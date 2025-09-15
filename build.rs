extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    println!("cargo:rustc-link-search=.");

    println!("cargo:rustc-link-lib=hello");

    println!("cargo:rerun-if-changed=c_src/hello.h");

    let bindings = bindgen::Builder::default()
        .header("c_src/hello.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // miette
    let path = std::path::PathBuf::from("src");

    let mut b = autocxx_build::Builder::new("src/autocxx.rs", &[path])
        .build()
        .expect("Failed to build");

    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/autocxx.rs");
    println!("cargo:rerun-if-changed=src/input.h");

    Ok(())
}
