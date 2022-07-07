mod context;
mod utils;

use detour::static_detour;
use std::ffi::c_void;
use std::sync::Once;
use std::thread;
use utils::d3d11;
use windows::core::HRESULT;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;
use windows::Win32::System::LibraryLoader;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

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

fn present(swap_chain: *const IDXGISwapChain, sync_internal: u32, flags: u32) -> HRESULT {
    PRESENT_DETOUR.call(swap_chain, sync_internal, flags)
}
