fn main() {
    let out_dir     : String = std::env::var("OUT_DIR").unwrap();
    let out_ckr_dir : String = format!("{out_dir}/ckr");
    let mut c : bool = false;

    println!("cargo:rerun-if-env-changed=CHAKRACORE_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=CHAKRACORE_LIB_DIR");

    match std::env::var("CHAKRACORE_INCLUDE_DIR") {
        Ok( _) => {}
        Err(_) =>  { c = true; }
    }

    match std::env::var("CHAKRACORE_LIB_DIR") {
        Ok(_) => {}
        Err(_) =>  { c = true; }
    }

    if c {
        let _ = std::process::Command::new("git")
            .args(["clone", "https://github.com/chakra-core/ChakraCore", out_ckr_dir.as_str(), "--depth", "1"])
            .output()
            .expect("Failed to fetch ChakraCore: https://github.com/chakra-core/ChakraCore");

        let dst = cmake::Config::new(out_ckr_dir)
            .define("CMAKE_C_COMPILER", "clang")
            .define("CMAKE_CXX_COMPILER", "clang++")
            .define("CMAKE_ASM_COMPILER", "clang")
            .env("NUM_JOBS", "30")
            .build_target("ChakraCore")
            .build()
            ;

        println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
        println!("cargo:rustc-link-lib=dylib=ChakraCore");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=icuuc");
        println!("cargo:rustc-link-lib=dylib=icui18n");
    }

    /* TODO:
     * what I've written is a fallback when library is not found.
     * cmake build is unnecessary when include & lib dir has been defined.
     * 
     * since there's no such standard way to link libraries
     * linking from env would be suitable.
     *
     * I suggest to use CHAKRACORE_LIB_PATH.
     * */
}
