extern crate bindgen;
extern crate fs_extra;

use std::env;
use std::path::PathBuf;

use std::process::Command;
use fs_extra::dir::{copy, CopyOptions};



fn out_path(p: &str) -> String {
    let out_dir : PathBuf = PathBuf::from(env::var("OUT_DIR").expect("no OUT_DIR specified"));
    let result = out_dir.join(p).to_str().expect("Couldn't convert path to string").to_string();
    println!("{}", result);
    result
}

fn build() {
    let target_vendor_path = out_path("target/vendor");

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    copy("vendor", &target_vendor_path, &options)
    .expect("Couldn't copy vendor to target");

    // we run make we that we can compile ruby libs to c
    Command::new("./minirake")
        .current_dir(&target_vendor_path)
        .status()
        .expect("Couldn't build mruby");

    println!("cargo:rustc-link-lib=static=mruby");
    println!("cargo:rustc-link-search=native={}/build/host/lib", target_vendor_path);
}

fn bindgen() {

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .opaque_type("mrb_heap_page") // tests were failing, this might be fixed in newer bindgen
        .generate_comments(false) // doctests were failing because they coppied C code...
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    build();
    bindgen();
}
