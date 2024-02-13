use std::{env, path::PathBuf};

fn main() {
    let in_dir = PathBuf::from(
        /* env::var("JAVA_HOME").expect("JAVA_HOME is not defined") */ "/lib/jvm/default",
    );
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header(in_dir.join("include/jni.h").to_str().unwrap())
        .clang_arg(format!(
            "-I{}",
            in_dir.join("include/linux/").to_str().unwrap()
        ))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings");
}
