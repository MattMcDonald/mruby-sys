extern crate bindgen;

use std::env;
use std::path::PathBuf;

use std::process::Command;



fn out_path(p: &str) -> String {
    let out_dir : PathBuf = PathBuf::from(env::var("OUT_DIR").expect("no OUT_DIR specified"));
    out_dir.join(p).to_str().expect("Couldn't convert path to string").to_string()
}

fn build() {

    Command::new("cp")
        .arg("-r")
        .arg("vendor")
        .arg(out_path("target"))
        .status()
        .expect("Couldn't copy vendor to target");

    // we run make we that we can compile ruby libs to c
    Command::new("./minirake")
        .current_dir(out_path("target/vendor"))
        .status()
        .expect("Couldn't build mruby");

    println!("cargo:rustc-link-lib=static=mruby");
    println!("cargo:rustc-link-search=native={}",out_path("target/vendor/build/host/lib"));
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
