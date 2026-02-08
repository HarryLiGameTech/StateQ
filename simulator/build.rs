use std::path::PathBuf;

fn main() {
    let debug = std::env::var("PROFILE").unwrap() == "debug";
    let quest_build = cmake::Config::new("QuEST/QuEST")
        .define("SHARED", "0")
        .cflag("-O3")
        // .define("CMAKE_C_COMPILER", "/usr/bin/gcc-12")
        // .define("CUDA_TOOLKIT_ROOT_DIR", "/opt/cuda")
        // .define("GPU_COMPUTE_CAPABILITY", "80")
        // .define("GPUACCELERATED", "1")
        .target("QuEST")
        .no_build_target(true)
        .build();
    let sources = std::fs::read_dir("src").unwrap().filter_map(|dir| {
        let dir = dir.unwrap();
        if dir.file_name().to_str().unwrap().ends_with(".cpp") {
            Some(dir.path())
        } else {
            None
        }
    }).collect::<Vec<PathBuf>>();
    cc::Build::new()
        .files(sources)
        .include("src")
        .include("QuEST/QuEST/include")
        .define("LOGLEVEL", if debug { "LogLevel::Info" } else { "LogLevel::Warning" })
        .define("qivm_available_qubits", "_qivm_available_qubits")
        .define("qivm_is_gate_available", "_qivm_is_gate_available")
        .define("qivm_exec_bytecode", "_qivm_exec_bytecode")
        .flag("-std=c++20")
        .flag("-O3")
        .compile("qivmbesim");
    println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN");
    println!("cargo:rustc-link-search={}/build", quest_build.to_str().unwrap());
    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-search=/opt/cuda/lib64/");
    println!("cargo:rustc-link-lib=static=qivmbesim");
    println!("cargo:rustc-link-lib=static=QuEST");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=rt");
    println!("cargo:rustc-link-lib=static=cudart_static");
    println!("cargo:rustc-link-lib=gomp");
}
