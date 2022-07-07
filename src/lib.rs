mod context;
mod utils;

use context::Context;
use detour::static_detour;
use imgui::ConfigFlags;
use std::ffi::c_void;
use std::sync::Once;
use std::{mem, thread};
use utils::d3d11;
use windows::core::HRESULT;
use windows::Win32::Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;
use windows::Win32::System::LibraryLoader;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{GWLP_WNDPROC, WNDPROC};

static INIT: Once = Once::new();
static mut CTX: Option<Context> = None;

static_detour! {
    static PRESENT_DETOUR: extern "stdcall" fn(*const IDXGISwapChain, u32, u32) -> HRESULT;
}

#[no_mangle]
pub extern "stdcall" fn DllMain(dll: HINSTANCE, reason: u32, _reserved: *const c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            LibraryLoader::DisableThreadLibraryCalls(dll);
        }

        thread::spawn(move || -> anyhow::Result<()> {
            let present_origin = d3d11::present()?;

            unsafe {
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
    unsafe {
        match CTX.as_ref() {
            None => WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam),
            Some(ctx) => {
                if utils::imgui::wnd_proc(hwnd, msg, wparam, lparam).0 != 0 {
                    return LRESULT(true.into());
                }

                return WindowsAndMessaging::CallWindowProcW(
                    ctx.wnd_proc,
                    hwnd,
                    msg,
                    wparam,
                    lparam,
                );
            }
        }
    }
}

fn present(swap_chain: *const IDXGISwapChain, sync_internal: u32, flags: u32) -> HRESULT {
    INIT.call_once(|| unsafe {
        let device = &*d3d11::device(swap_chain);
        let window = d3d11::desc(swap_chain).OutputWindow;
        let device_ctx = &*d3d11::immediate_context(device);

        let buf = &*d3d11::buf(swap_chain);
        let target_view = &*d3d11::create_render_target(device, buf);

        let wnd_proc = WindowsAndMessaging::SetWindowLongPtrW(
            window,
            GWLP_WNDPROC,
            wnd_proc as usize as isize,
        );
        let wnd_proc: WNDPROC = mem::transmute(wnd_proc);

        let mut imgui = imgui::Context::create();
        let io = imgui.io_mut();
        io.config_flags = ConfigFlags::NO_MOUSE_CURSOR_CHANGE;

        utils::imgui::init(window, device, device_ctx);

        let ctx = Context::new(imgui, device_ctx, target_view, wnd_proc);
        CTX = Some(ctx);
    });

    unsafe {
        match CTX.as_mut() {
            None => {
                let _ = PRESENT_DETOUR.disable();
            }
            Some(ctx) => {
                utils::imgui::frame();

                let ui = ctx.imgui.frame();
                ui.show_demo_window(&mut true);
                let draw_data = ui.render();

                d3d11::render_target(ctx.device_ctx, ctx.target_view);
                utils::imgui::render(draw_data);
            }
        }
    }

    PRESENT_DETOUR.call(swap_chain, sync_internal, flags)
}
