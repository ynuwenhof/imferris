use windows::Win32::Graphics::Direct3D11::{ID3D11DeviceContext, ID3D11RenderTargetView};
use windows::Win32::UI::WindowsAndMessaging::WNDPROC;

pub struct Context<'a> {
    pub imgui: imgui::Context,
    pub device_ctx: &'a ID3D11DeviceContext,
    pub target_view: &'a ID3D11RenderTargetView,
    pub wnd_proc: WNDPROC,
}

impl<'a> Context<'a> {
    pub fn new(
        imgui: imgui::Context,
        device_ctx: &'a ID3D11DeviceContext,
        target_view: &'a ID3D11RenderTargetView,
        wnd_proc: WNDPROC,
    ) -> Self {
        Self {
            imgui,
            device_ctx,
            target_view,
            wnd_proc,
        }
    }
}

unsafe impl<'a> Send for Context<'a> {}
unsafe impl<'a> Sync for Context<'a> {}
