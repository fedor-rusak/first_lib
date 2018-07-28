# first_lib

This is some complex library for playing with low-level graphics API under Windows.

# Windows instructions from scratch

 * Install Rust, Cargo with [rustup](https://www.rust-lang.org/en-US/install.html)
 * Install [Visual C++ Builds Tools 2015 or later](https://visualstudio.microsoft.com/ru/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15)

Run these commands

```
cargo build
cargo test
```

PS Behind proxy set environment variables *http_proxy* and *https_proxy*.

# Libraries used

 * [GLFW](https://github.com/glfw/glfw) - zlib/libpng License
 * [GLEW](https://github.com/nigels-com/glew) -  Modified BSD License, Mesa 3-D License (MIT License), and the Khronos License (MIT License)
 * [Vulkan loader](https://www.lunarg.com/vulkan-sdk/) - Apache License, Version 2.0

# History

## 0.1.2
  - added stuff for Vulkan API

## 0.1.1
  - native stuff + DLLs for MSVC 64. Many changes...

## 0.1.0
  - initial version with funny method and test.