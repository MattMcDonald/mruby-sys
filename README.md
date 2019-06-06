# mruby-sys
Low level mruby bindings for rust

The crate will build mruby and statically link it into your Rust application. Currently it uses **mruby 2.0.1** included as a submodule in the vendor/ directory.

This is currently a very early WIP so very basic things might not work, or worse they may appear to work but do very unsafe things. There is a simple hello world test that runs on my machine.

## How it works
Most of the work happens in the build.rs script which:

1. Copies the mruby source in the vendor/ directory into the target/ directory
2. Builds mruby in the target/ directory using mruby's minirake based build system
3. Uses bindgen to generate Rust bindings based on the wrapper.h file in the include/ directory. These bindings and their tests are rexported out in src/lib.rs.


## Todo list:
- Enable support for choosing gems
- Support toolchains besides clang
- Reasonable support for passing along compiler flags to the minirake build, especially debug vs release builds.
- Web Assembly?!?
- Reasonable support for other mruby build options, i.e. building with no gem support

## Release Notes

### 0.2.0
- Generating bindings for string.h

### 0.1.1
- Fixed build paths so that other crates can actually depend on us

### 0.1.0
- Initial build with hello world working

## License
Copyright © 2019, Matthew McDonald. Released under the MIT License.