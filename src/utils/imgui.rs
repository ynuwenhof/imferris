use imgui::DrawData;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11DeviceContext};

extern "C" {
    pub fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    pub fn init(window: HWND, device: *const ID3D11Device, ctx: *const ID3D11DeviceContext);
    pub fn frame();
    pub fn render(draw_data: *const DrawData);
}
