extern crate cc;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    fs::copy(
        &Path::new("src/peanut_gb/peanut_gb.h"),
        &Path::new("src/peanut_gb/peanut_gb.c"),
    )
    .expect("Failed to copy file");

    let toolchain = PathBuf::from("/home/igor-wsl/pfc/cashew-juice-espgb/.embuild/espressif/tools/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/bin/xtensa-esp32s3-elf-gcc");
    cc::Build::new()
        .file("src/peanut_gb/peanut_gb.c")
        .compiler(toolchain)
        .flag("-mlongcalls")
        // .flag("-g2")
        .flag("-O2")
        .flag("-std=c99")
        .compile("peanut_gb");

    println!("cargo:rerun-if-changed=src/peanut_gb/peanut_gb.c");
    println!("cargo:rerun-if-changed=src/peanut_gb/peanut_gb.h");

    embuild::espidf::sysenv::output();
}
