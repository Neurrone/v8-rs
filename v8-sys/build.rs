// use cc;
// extern crate cc;
#[cfg(feature = "bindgen")]
extern crate bindgen;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let v8_libs_dir = env::var("V8_LIBS").unwrap_or_else(|_| {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("lib")
            .to_string_lossy()
            .to_string()
    });
    let mut libs = vec![
        "inspector",
        "v8_base_0",
        "v8_base_1",
        "v8_external_snapshot",
        "v8_init",
        "v8_initializers",
        "v8_libbase",
        "v8_libplatform",
        "v8_libsampler",
    ];
    // let mut libs = vec!["inspector", "v8_base_0", "v8_base_1", "v8_init", "v8_initializers", "v8_libbase", "v8_libplatform", "v8_libsampler", "v8_nosnapshot", "libcmtd", "torque_base", "torque_generated_initializers"];
    if cfg!(windows) {
        // V8 also requires these
        libs.push("dbghelp");
        libs.push("shlwapi");
        libs.push("winmm");
        // temporarily link to the MSVC debug multithreaded CRT to debug context crashing
        libs.push("libcmtd");
    }

    for l in &libs {
        println!("cargo:rustc-link-lib=dylib={}", l);
    }
    println!("cargo:rustc-link-search={}", v8_libs_dir);
    /*    
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .include(v8_includes_dir)
        .file("src/allocator.cpp")
        .file("src/isolate.cpp")
        .file("src/platform.cpp")
        .compile("librust-v8-impls.a");
    
    // println!("cargo:rustc-link-lib=dylib=librust-v8-impls.a");
    */

    #[cfg(feature = "bindgen")]
    {
        generate_bindings();
    }

    #[cfg(not(feature = "bindgen"))]
    {
        copy_pregenerated_bindings();
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    use std::path;
    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("src/wrapper.hpp")
        .rust_target(bindgen::RustTarget::Nightly)
        .clang_arg("--std=c++14")
        // Because there are some layout problems with these
        .opaque_type("std::.*")
        .whitelist_type("std::unique_ptr\\<v8::Platform\\>")
        .whitelist_type("v8::.*")
        .whitelist_type("rust_v8_impls::.*")
        .whitelist_function("v8::.*")
        .whitelist_function("rust_v8_impls::.*")
        .whitelist_var("v8::.*")
        .whitelist_var("rust_v8_impls::.*")
        // Re-structure the modules a bit and hide the "root" module
        .raw_line("#[doc(hidden)]")
        // .generate_inline_functions(true)
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate v8 bindings");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings file");

    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        out_path.join("bindings.rs"),
        crate_path.join("pregenerated_bindings.rs"),
    ).expect("Couldn't find generated bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    ).expect("Couldn't find pregenerated bindings!");
}
