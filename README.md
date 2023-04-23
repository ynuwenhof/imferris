# Imferris

A proof of concept internal DirectX 11 [ImGui](https://github.com/ocornut/imgui) menu written in [Rust](https://www.rust-lang.org).

Imferris utilizes the [imgui_impl_win32.cpp](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_win32.cpp) platform and [imgui_impl_dx11.cpp](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_dx11.cpp) renderer via Rust to C++ interop.

## Usage

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

```bash
git clone --recurse-submodules https://github.com/ynuwenhof/imferris.git
cd imferris
rustup default nightly
cargo b --release
```

After building, you can inject the DLL into the target process.

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/imferris/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/imferris/blob/main/LICENSE-MIT))

![showcase](https://user-images.githubusercontent.com/100025337/177890937-fe4e0001-6f0f-4cc1-8af5-5e1e0bdb524f.png)
