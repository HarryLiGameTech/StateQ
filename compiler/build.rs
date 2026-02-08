extern crate cc;

use std::{env, fs};
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(unix)]
fn build_stateq_library() {
    let lib_dir = env::current_dir().unwrap();
    let output = Command::new("/bin/sh")
        .current_dir(&lib_dir)
        .arg(format!("{}/gradlew", lib_dir.to_str().unwrap()))
        .arg("nativeBuild")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .expect("Fail to build stateq compiler library");
    eprintln!("{}", String::from_utf8(output.stdout).unwrap());
}

#[cfg(windows)]
fn build_stateq_library() {
    let lib_dir = env::current_dir().unwrap().join("lib");
    Command::new("cmd")
        .current_dir(&lib_dir)
        .arg(format!("{}/gradlew.bat", lib_dir.to_str().unwrap()))
        .arg("nativeBuild")
        .stderr(Stdio::inherit())
        .spawn().expect("Fail to build stateq compiler library");
}

const LIBSTATEQ_BUILD_DIR: &str = "build/native/nativeCompile";

fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN");
    println!("cargo:rustc-link-search={}", LIBSTATEQ_BUILD_DIR);
    println!("cargo:rerun-if-changed=src/main/*");
    println!("cargo:rustc-link-lib=static=qcinterface");
    println!("cargo:rustc-link-lib=dylib=stateq");
    build_stateq_library();
    cc::Build::new()
        .file("src/main/native/stateq_compiler.c")
        .include(LIBSTATEQ_BUILD_DIR)
        .object(format!("{}/libstateq.so", LIBSTATEQ_BUILD_DIR))
        .compile("qcinterface");
    // copy `libstateq.so` to target directory
    // TODO: do this only in `install` target
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = env::current_dir().unwrap()
        .join(format!("{}/libstateq.so", LIBSTATEQ_BUILD_DIR));
    println!("{}", out_dir);
    let dest_path = Path::new(&out_dir).join("libstateq.so");
    fs::copy(lib_path, dest_path)
        .expect("Failed to copy stateq compiler library to target directory");
}
