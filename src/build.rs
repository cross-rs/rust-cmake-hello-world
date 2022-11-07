use cmake;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");

    build_sum();
    build_re2();
}

fn build_sum() {
    let dst = cmake::build("libsum");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=sum");
}

fn build_re2() {
    if cfg!(feature = "re2") {
        let mut config = cmake::Config::new("re2");
        let abi = env::var("CARGO_CFG_TARGET_ENV");
        if env::var("CARGO_CFG_WINDOWS").is_ok() && matches!(abi.as_deref(), Ok("gnu")) {
            // rust needs windows 7+, and re2 needs windows vista+
            config
                .cflag("-DWINVER=0x0601")
                .cflag("-D_WIN32_WINNT=0x0601")
                .cxxflag("-DWINVER=0x0601")
                .cxxflag("-D_WIN32_WINNT=0x0601");
        }
        let dst = config.build();

        let mut build = cc::Build::new();

        // msvc doesn't support C++11, only C++14 and higher, but
        // older targets like CentOS have only partial C++11 support.
        match abi.as_deref() {
            Ok("msvc") => build.flag("-std=c++14"),
            _ => build.flag("-std=c++11"),
        };

        build
            .cpp(true)
            .file("match.cc")
            .include(dst.join("include"))
            .compile("libmatch.a");

        println!("cargo:rerun-if-changed=match.cc");
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=re2");
    }
}
