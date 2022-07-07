mod utils;

use std::ffi::c_void;
use windows::core::HRESULT;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

#[no_mangle]
pub extern "stdcall" fn DllMain(_dll: HINSTANCE, _reason: u32, _reserved: *const c_void) -> BOOL {
    true.into()
}

fn present(_swap_chain: *const IDXGISwapChain, _sync_internal: u32, _flags: u32) -> HRESULT {
    todo!()
}
