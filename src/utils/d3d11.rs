use anyhow::anyhow;
use std::{mem, ptr};
use windows::core::HRESULT;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11;
use windows::Win32::Graphics::Direct3D11::{D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::UI::WindowsAndMessaging;

pub type Present = extern "stdcall" fn(*const IDXGISwapChain, u32, u32) -> HRESULT;

pub fn present() -> anyhow::Result<Present> {
    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ..Default::default()
        },
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 1,
        OutputWindow: unsafe { WindowsAndMessaging::GetDesktopWindow() },
        Windowed: true.into(),
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        ..Default::default()
    };

    let mut swap_chain = None;

    unsafe {
        Direct3D11::D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_FLAG::default(),
            &[],
            D3D11_SDK_VERSION,
            &swap_chain_desc,
            &mut swap_chain,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )?;
    }

    let swap_chain = swap_chain.ok_or_else(|| anyhow!("Failed to create swap chain"))?;

    let present: Present = unsafe {
        let vmt = **((&swap_chain as *const _) as *const *const *const *const ());
        mem::transmute(*vmt.offset(8))
    };

    Ok(present)
}
