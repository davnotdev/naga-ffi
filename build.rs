use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("naga.h")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-DNAGA_FFI_NO_METHODS")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
