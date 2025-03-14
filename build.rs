use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let src = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    build_apl_wrapper(&src);
    bindgen_apl_wrapper(&src, &out)?;
    Ok(())
}

fn build_apl_wrapper(src: &Path) {
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include(src.join("cpp"))
        .file(src.join("cpp/apl_wrapper.cpp"))
        .compile("aplwrapper");
}

fn bindgen_apl_wrapper(src: &Path, out: &Path) -> std::io::Result<()> {
    bindgen::Builder::default()
        .header(src.join("cpp/apl_wrapper.hpp").to_string_lossy())
        .allowlist_item("aplw_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(if cfg!(windows) {
            "/std:c++17"
        } else {
            "-std=c++17"
        })
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out.join("aplw_bindings.rs"))
}
