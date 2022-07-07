use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("interop.cpp")
        .include("imgui")
        .file("imgui/backends/imgui_impl_dx11.cpp")
        .file("imgui/backends/imgui_impl_win32.cpp")
        .compile("interop");
}
