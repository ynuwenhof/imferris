use windows::core::HRESULT;
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

pub type Present = extern "stdcall" fn(*const IDXGISwapChain, u32, u32) -> HRESULT;
