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

    // we run make we that we can compile ruby libs to c
    Command::new("make")
        .current_dir("target/vendor")
        .status()
        .unwrap();

    let mut build = cc::Build::new();
    

    build.include("target/vendor/include");

    build.file("target/vendor/build/host/mrblib/mrblib.c");

    for entry in glob("target/vendor/src/*.c").unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display());
            build.file(path);
        }
    }

    for entry in glob("target/vendor/mrblib/*.c").unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display());
            build.file(path);
        }
    }

    let gems = vec![""];

for gem in gems{
    let path = format!("target/vendor/mrbgems/{}/src/*.c", gem);
    for entry in glob(&path).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display());
            build.file(path);
        }
    }
}


    build.warnings(false);
    build.static_flag(true);
    build.define("DISABLE_GEMS", "TRUE");
    build.compile("mruby");
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
