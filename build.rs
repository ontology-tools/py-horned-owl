use std::env;

pub fn main() {
    println!("cargo::rustc-check-cfg=cfg(pyi)");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=INCLUDE_PYI");
    if let Ok(flag) = env::var("INCLUDE_PYI") {
        if flag == "1" || flag.to_lowercase() == "true" {
            println!("cargo:rustc-cfg=pyi");
        }
    }
}
