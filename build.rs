use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=faddeeva/Faddeeva.cc");
    println!("cargo:rerun-if-changed=faddeeva/Faddeeva.hh");

    cc::Build::new()
        .file("faddeeva/Faddeeva.cc")
        .compile("faddeeva");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("faddeeva/Faddeeva.hh")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("Faddeeva::.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldnt write bindings.")
}
