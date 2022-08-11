# imferris

Proof of concept internal DirectX 11 [ImGui](https://github.com/ocornut/imgui) menu written in [Rust](https://www.rust-lang.org/).

Imferris utilizes the [imgui_impl_win32.cpp](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_win32.cpp) platform and [imgui_impl_dx11.cpp](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_dx11.cpp) renderer via Rust to C++ interop.

## Usage

1. Install Rust from [here](https://www.rust-lang.org/)
2. Clone the repo and cd into the directory

    ```
    git clone https://github.com/ynuwenhof/imferris
    cd imferris
    ```
3. Build the DLL by running `cargo b --release`
4. Inject the DLL into the target process using the DLL injector of your choice

![showcase](https://user-images.githubusercontent.com/100025337/177890937-fe4e0001-6f0f-4cc1-8af5-5e1e0bdb524f.png)
