use cmake;

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
        let dst = cmake::build("re2");

        cc::Build::new()
            .cpp(true)
            .file("match.cc")
            .include(dst.join("include"))
            .compile("libmatch.a");

        println!("cargo:rerun-if-changed=match.cc");
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=re2");
    }
}
