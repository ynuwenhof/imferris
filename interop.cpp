#include <d3d11.h>
#include <Windows.h>
#include "imgui/imgui.h"
#include "imgui/backends/imgui_impl_dx11.h"
#include "imgui/backends/imgui_impl_win32.h"

extern LRESULT ImGui_ImplWin32_WndProcHandler(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);

extern "C" {
    LRESULT wnd_proc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
        return ImGui_ImplWin32_WndProcHandler(hwnd, msg, wParam, lParam);
    }

    void init(HWND window, ID3D11Device* device, ID3D11DeviceContext* ctx) {
        ImGui_ImplWin32_Init(window);
        ImGui_ImplDX11_Init(device, ctx);
    }

    void frame() {
        ImGui_ImplDX11_NewFrame();
        ImGui_ImplWin32_NewFrame();
    }

    void render(ImDrawData* draw_data) {
        ImGui_ImplDX11_RenderDrawData(draw_data);
    }

    ID3D11Device* device(IDXGISwapChain* swap_chain) {
        ID3D11Device* device;
        swap_chain->GetDevice(__uuidof(ID3D11Device), (void**)&device);
        return device;
    }

    ID3D11DeviceContext* immediate_context(ID3D11Device* device) {
        ID3D11DeviceContext* context;
        device->GetImmediateContext(&context);
        return context;
    }

    DXGI_SWAP_CHAIN_DESC desc(IDXGISwapChain* swap_chain) {
        DXGI_SWAP_CHAIN_DESC swap_chain_desc {0};
        swap_chain->GetDesc(&swap_chain_desc);
        return swap_chain_desc;
    }

    ID3D11Texture2D* buf(IDXGISwapChain* swap_chain) {
        ID3D11Texture2D* buf;
        swap_chain->GetBuffer(0, __uuidof(ID3D11Texture2D), (void**)&buf);
        return buf;
    }

    ID3D11RenderTargetView* create_render_target(ID3D11Device* device, ID3D11Texture2D* buf) {
        ID3D11RenderTargetView* target_view;
        device->CreateRenderTargetView(buf, NULL, &target_view);
        return target_view;
    }

    void render_target(ID3D11DeviceContext* context, ID3D11RenderTargetView* target_view) {
        context->OMSetRenderTargets(1, &target_view, NULL);
    }
}
