#![feature(stmt_expr_attributes)]
#![feature(local_key_cell_methods)]

mod utils;

use anyhow::anyhow;
use imgui::{ConfigFlags, Key};
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::Mutex;
use retour::static_detour;
use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::Once;
use std::{mem, thread};
use utils::d3d11;
use windows::core::HRESULT;
use windows::Win32::Foundation::{BOOL, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Direct3D11::{ID3D11DeviceContext, ID3D11RenderTargetView};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;
use windows::Win32::System::LibraryLoader;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{GWLP_WNDPROC, HCURSOR, IDC_ARROW, WNDPROC};

static INIT: Once = Once::new();
static WND_PROC: OnceCell<WNDPROC> = OnceCell::new();
static DEVICE: OnceCell<&ID3D11DeviceContext> = OnceCell::new();
static TARGET_VIEW: OnceCell<&ID3D11RenderTargetView> = OnceCell::new();
static ENABLED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

#[link(name = "windows")]
extern "system" {
    fn SetCursor(cursor: HCURSOR) -> HCURSOR;
    fn SetCursorPos(x: i32, y: i32) -> BOOL;
}

thread_local! {
    static IMGUI: RefCell<Option<imgui::Context>> = RefCell::new(None);
}

static_detour! {
    static SET_CURSOR_DETOUR: unsafe extern "system" fn(HCURSOR) -> HCURSOR;
    static SET_CURSOR_POS_DETOUR: unsafe extern "system" fn(i32, i32) -> BOOL;
    static PRESENT_DETOUR: unsafe extern "stdcall" fn(*const IDXGISwapChain, u32, u32) -> HRESULT;
}

#[no_mangle]
pub extern "stdcall" fn DllMain(dll: HMODULE, reason: u32, _reserved: *const c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            LibraryLoader::DisableThreadLibraryCalls(dll);
        }

        thread::spawn(move || -> anyhow::Result<()> {
            let present_origin = d3d11::present()?;

            unsafe {
                SET_CURSOR_DETOUR
                    .initialize(SetCursor, set_cursor)?
                    .enable()?;

                SET_CURSOR_POS_DETOUR
                    .initialize(SetCursorPos, set_cursor_pos)?
                    .enable()?;

                PRESENT_DETOUR
                    .initialize(present_origin, present)?
                    .enable()?;
            }

            Ok(())
        });
    }

    true.into()
}

fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if unsafe { utils::imgui::wnd_proc(hwnd, msg, wparam, lparam).0 } != 0 {
        return LRESULT(true.into());
    }

    {
        let enabled = ENABLED.lock();

        if *enabled {
            unsafe {
                if let Ok(cursor) = WindowsAndMessaging::LoadCursorW(None, IDC_ARROW) {
                    SET_CURSOR_DETOUR.call(cursor);
                }
            }
        } else if let Some(wnd_proc) = WND_PROC.get() {
            drop(enabled);

            return unsafe {
                WindowsAndMessaging::CallWindowProcW(*wnd_proc, hwnd, msg, wparam, lparam)
            };
        }
    }

    unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn set_cursor(cursor: HCURSOR) -> HCURSOR {
    {
        let enabled = ENABLED.lock();
        if *enabled {
            return HCURSOR(0);
        }
    }

    unsafe { SET_CURSOR_DETOUR.call(cursor) }
}

fn set_cursor_pos(x: i32, y: i32) -> BOOL {
    {
        let enabled = ENABLED.lock();
        if *enabled {
            drop(enabled);
            return false.into();
        }
    }

    unsafe { SET_CURSOR_POS_DETOUR.call(x, y) }
}

fn present(swap_chain: *const IDXGISwapChain, sync_internal: u32, flags: u32) -> HRESULT {
    INIT.call_once(|| {
        let init = || -> anyhow::Result<()> {
            let swap_chain = unsafe { swap_chain.as_ref() }
                .ok_or_else(|| anyhow!("Failed to get reference to swap chain"))?;

            let (device, target_view) = unsafe {
                let device = d3d11::device(swap_chain)
                    .as_ref()
                    .ok_or_else(|| anyhow!("Failed to get d3d11 device"))?;

                let buf = d3d11::buf(swap_chain)
                    .as_ref()
                    .ok_or_else(|| anyhow!("Failed to get d3d11 buffer"))?;

                let target_view = d3d11::create_render_target(device, buf)
                    .as_ref()
                    .ok_or_else(|| anyhow!("Failed to create d3d11 target view"))?;

                (device, target_view)
            };

            let window = unsafe { d3d11::desc(swap_chain) }.OutputWindow;

            let wnd_proc: WNDPROC = unsafe {
                let wnd_proc = WindowsAndMessaging::SetWindowLongPtrW(
                    window,
                    GWLP_WNDPROC,
                    wnd_proc as usize as isize,
                );

                mem::transmute(wnd_proc)
            };

            let mut imgui = imgui::Context::create();
            let io = imgui.io_mut();
            io.config_flags = ConfigFlags::NO_MOUSE_CURSOR_CHANGE;

            let device = unsafe {
                let device_ctx = d3d11::immediate_context(device)
                    .as_ref()
                    .ok_or_else(|| anyhow!("Failed to get d3d11 context"))?;

                utils::imgui::init(window, device, device_ctx);

                device_ctx
            };

            WND_PROC.get_or_init(|| wnd_proc);
            DEVICE.get_or_init(|| device);
            TARGET_VIEW.get_or_init(|| target_view);

            IMGUI.with(|f| {
                *f.borrow_mut() = Some(imgui);
            });

            Ok(())
        };

        let _ = init();
    });

    IMGUI.with_borrow_mut(|f| {
        #[rustfmt::skip]
        if let (
            Some(imgui),
            Some(device),
            Some(target_view)
        ) = (f, DEVICE.get(), TARGET_VIEW.get()) {
            unsafe {
                utils::imgui::frame();
            }

            let ui = imgui.frame();

            {
                let mut enabled = ENABLED.lock();

                if ui.is_key_pressed(Key::Insert) {
                    *enabled = !*enabled;
                }

                if *enabled {
                    ui.show_demo_window(&mut enabled);
                }
            }

            let draw_data = ui.render();

            unsafe {
                d3d11::render_target(*device, *target_view);
                utils::imgui::render(draw_data);
            }
        }
    });

    unsafe { PRESENT_DETOUR.call(swap_chain, sync_internal, flags) }
}
