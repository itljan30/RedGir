use cmake;

fn main() {
    if cfg!(feature = "cpp-bindings") {
        let dst = cmake::Config::new("src/ffi/cpp")
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib=static=RedGir_cpp");
    }
}
