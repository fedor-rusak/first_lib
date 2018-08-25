# render_lib

Not so simple rust library for playing with low-level graphics API under Windows.

# Windows instructions from scratch

 * Install Rust, Cargo with [rustup](https://www.rust-lang.org/en-US/install.html)
 * Install [Visual C++ Builds Tools 2015 or later](https://visualstudio.microsoft.com/ru/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15)
 * Install win32 port of [GNU Make system](https://sourceforge.net/projects/gnuwin32/files/make/). Even old version will be sufficient. Don't forget to add its bin folder in PATH.

Run this in CMD while being in Root folder to test library:

```
make clean build test
```

Run this to get documentation in *target/doc/render_lib/index.html* about this library:
```
make doc
```

PS Behind proxy set environment variables *http_proxy* and *https_proxy*.

# Libraries used

 * [GLFW](https://github.com/glfw/glfw) - zlib/libpng License
 * [GLEW](https://github.com/nigels-com/glew) -  Modified BSD License, Mesa 3-D License (MIT License), and the Khronos License (MIT License)
 * [Vulkan loader](https://www.lunarg.com/vulkan-sdk/) - Apache License, Version 2.0

# History

## 0.1.3
  - refactored module structure. Now renderers are public API.

## 0.1.2
  - added stuff for Vulkan API

## 0.1.1
  - native stuff + DLLs for MSVC 64. Many changes...

## 0.1.0
  - initial version with funny method and test.