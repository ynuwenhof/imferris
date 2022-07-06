mod utils;

use std::ffi::c_void;
use windows::Win32::Foundation::{BOOL, HINSTANCE};

#[no_mangle]
pub extern "stdcall" fn DllMain(_dll: HINSTANCE, _reason: u32, _reserved: *const c_void) -> BOOL {
    true.into()
}
