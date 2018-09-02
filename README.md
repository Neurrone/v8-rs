# V8-rs: An idiomatic Rust wrapper for the V8 Javascript engine

Rust bindings for [v8][v8] 7.0, the JavaScript engine.

This is an attempt to rewrite the existing [v8 crate](https://github.com/dflemstr/v8-rs) to enable it to work with the latest version of V8. Hence, substantial portions of this code was copied verbatim whenever possible.

## Documentation

Use `cargo doc --open`. This is not on crates.io yet, since this is highly experimental, and an existing crate already exists with this name.

## Requirements

* Llatest nightly Rust.
* Static libraries for V8 built with snapshot support and without i18n support for now. V8-sys will  look for the necessary .libs to link against in the `V8_LIBS` environment variable if defined, or v8-sys's `CARGO_MANIFEST_DIR/lib` folder otherwise.
* Place `natives_blob.bin` and `snapshot_blob.bin` next to  executables (currently only `main.rs`) using this crate.

## Installation

Clone from GitHub to obtain the latest development version, ensure you have V8 static libraries, then `cargo build`.

## Hints for building V8 statically on Windows

Building on Windows is very frustrating due to some [known bugs](https://bugs.chromium.org/p/v8/issues/detail?id=8119). Here's how to get a successful build.

1. Follow the steps in the official documentation to check out the V8 repository and set up depot tools and other dependencies. The latest version of VS2017 works as well. 

2. Edit the `args.gn` file in the `out.gn/x64-release` folder so it contains the following:

```sh
is_debug = false
target_cpu = "x64"
is_component_build = false
v8_static_library = true
v8_enable_i18n_support=false
is_clang=false
```

3. Build the libraries for embedding with the following command:

```sh
ninja -C out.gn/x64.release v8
```

The official documentation suggests using the `ninja -C out.gn/x64.release`, which builds tests and fuzzers in addition to the libraries. Besides taking twice as long, the build fails due to unfixed bugs in the compilation of tests.

4. You will need to place the  following libraries from the build somewhere v8-sys can  link against: `inspector.lib, v8_base_0.lib, v8_base_1.lib, v8_external_snapshot.lib, v8_init.lib, v8_initializers.lib, v8_libbase.lib, v8_libplatform.lib, v8_libsampler.lib, v8_nosnapshot.lib`, and `v8_snapshot.lib`.

Also see the [official V8 documentation](https://github.com/v8/v8/wiki/Building-from-Source) and [How to build V8 on Windows and not go mad](https://medium.com/dailyjs/how-to-build-v8-on-windows-and-not-go-mad-6347c69aacd4)

## Generating the v8-sys bindings with bindgen

The optional feature "use-bindgen" generates the low level bindings using the included V8 headers (version 7.0), which requires [bindgen](https://github.com/rust-lang-nursery/rust-bindgen) to be installed. Generating this shouldn't be necessary in most cases, as the bindings have already been bundled with this crate.

[crates]: https://crates.io/
[v8]: https://developers.google.com/v8/