extern crate bindgen;
extern crate cc;
extern crate glob;

use glob::glob;

use std::env;
use std::path::PathBuf;

use std::process::Command;

fn build() {
    Command::new("cp")
        .arg("-r")
        .arg("vendor")
        .arg("target")
        .status()
        .unwrap();

    let mut build = cc::Build::new();
    build.include("target/vendor/include");

    for entry in glob("target/vendor/src/*.c").unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display());
            build.file(path);
        }
    }

    build.compile("mruby");
    //     println!("cargo:rustc-link-search=native=target/vendor/build/host/lib");
    //     println!("cargo:rustc-link-lib=static=mruby");
    //     println!("cargo:rustc-link-lib=static=mruby_core");
}

fn bindgen() {
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .opaque_type("mrb_heap_page") // tests were failing, this might be fixed in newer bindgen
        .generate_comments(false) // doctests were failing because they coppied C code...
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    build();
    bindgen();
}
