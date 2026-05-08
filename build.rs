//! Build script for showpid
//! Performs platform detection and sets compile-time configuration

fn main() {
    // Print platform information during build
    println!("cargo:rustc-cfg=platform=\"{}\"", std::env::var("TARGET").unwrap());

    // Ensure we only build on supported platforms
    let target = std::env::var("TARGET").unwrap();
    if !(target.contains("windows") || target.contains("linux") || target.contains("apple")) {
        println!(
            "cargo:warning=showpid is designed for Windows, Linux, and macOS. Your target '{}' may not work correctly.",
            target
        );
    }
}
