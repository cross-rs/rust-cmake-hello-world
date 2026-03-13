use cmake;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");

    build_sum();
    build_tryrun();
    build_re2();
}

fn build_sum() {
    let dst = cmake::build("libsum");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=sum");
}

fn build_tryrun() {
    if cfg!(feature = "tryrun") {
        let _ = cmake::build("tryrun");
    }
}

fn build_re2() {
    if cfg!(feature = "re2") {
        let mut absl_config = cmake::Config::new("absl");
        let mut re2_config = cmake::Config::new("re2");
        let abi = env::var("CARGO_CFG_TARGET_ENV");
        if env::var("CARGO_CFG_WINDOWS").is_ok() && matches!(abi.as_deref(), Ok("gnu")) {
            // rust needs windows 7+, and re2 needs windows vista+
            absl_config
                .cflag("-DWINVER=0x0601")
                .cflag("-D_WIN32_WINNT=0x0601")
                .cxxflag("-DWINVER=0x0601")
                .cxxflag("-D_WIN32_WINNT=0x0601");
            re2_config
                .cflag("-DWINVER=0x0601")
                .cflag("-D_WIN32_WINNT=0x0601")
                .cxxflag("-DWINVER=0x0601")
                .cxxflag("-D_WIN32_WINNT=0x0601");
        }
        let absl_dst = absl_config.build();
        re2_config.define("CMAKE_PREFIX_PATH", absl_dst.display().to_string());
        let re2_dst = re2_config.build();

        let mut build = cc::Build::new();

        match abi.as_deref() {
            Ok("msvc") => build.flag("-std:c++17"),
            _ => build.flag("-std=c++17"),
        };

        build
            .cpp(true)
            .file("match.cc")
            .include(absl_dst.join("include"))
            .include(re2_dst.join("include"))
            .compile("libmatch.a");

        println!("cargo:rerun-if-changed=match.cc");

        // Use pkg-config to resolve re2 and all its absl dependencies
        let pc_path = format!(
            "{}/lib/pkgconfig:{}/lib/pkgconfig",
            absl_dst.display(),
            re2_dst.display()
        );
        env::set_var("PKG_CONFIG_PATH", &pc_path);
        pkg_config::Config::new()
            .statik(true)
            .probe("re2")
            .unwrap();
    }
}
