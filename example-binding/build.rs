fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = std::path::Path::new(&path).join("cpp");
    println!("cargo:warning=CPP_PATH={}", dir.display());

    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:warning=OUT_DIR={}", out_dir);

    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    println!("cargo:warning=LATEST_TS={}", now.as_secs_f64());

    // Check if the directory exists
    if !dir.exists() {
        panic!("Failed to properly resolve cpp/ dir: {}", dir.display());
    }
    
    // FIXED! The LTO causes the linking issue but only when:
    // - we are using the rust crate example-binding as a dependency 
    // - the platform is using --as-needed to link the library
    cc::Build::new()
        .cpp(true)
        .file(dir.join("stuff.cpp"))
        .include(&dir)
        .std("c++20")
        .opt_level(3)
        .flag_if_supported("-march=native")
        // .flag("-flto")  // <- This causes the linking issue
        .compile("stuff_from_cpp");
    
    println!("cargo:rerun-if-changed={}/stuff.h", dir.display());
    println!("cargo:rerun-if-changed={}/stuff.cpp", dir.display());
}