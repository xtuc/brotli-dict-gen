use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./Cargo.toml");
    println!("cargo:rerun-if-changed=./src/dictionary_generator.h");
    println!("cargo:rerun-if-changed=./src/dictionary_generator.c++");

    let bindings = bindgen::Builder::default()
        .header("./src/dictionary_generator.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .allowlist_function("generate")
        .allowlist_function("free_result")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = cmake::build("./third_party/libdivsufsort");
    println!("cargo:rustc-link-search={}/lib", dst.display());

    // Move the generated divsufsort.h to where durschlag expected it.
    {
        fs::create_dir_all(out_path.join("third_party/libdivsufsort/include/")).unwrap();
        fs::copy(
            out_path.join("include/divsufsort.h"),
            out_path.join("third_party/libdivsufsort/include/divsufsort.h"),
        )
        .unwrap();
    }

    cc::Build::new()
        .include(out_path)
        .cpp(true)
        .flag("-O3")
        .file("./brotli/research/durchschlag.cc")
        .file("./src/dictionary_generator.c++")
        .compile("dictionary_generator");
}
