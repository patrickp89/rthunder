//! # build.rs
//! A custom build script that compiles two thin C wrapper
//! libraries.

use std::process::Command;

/// The custom build: executes 'make' for the makefile
/// in the special folder "./native". This will build two
/// library wrappers for libcdio and libcddb respectively.
fn main() {
    let status_result = Command::new("make").current_dir("native/").status();

    match status_result {
        Ok(status) => {
            if !status.success() {
                panic!("'make' was not successful! Exit code was: {}", status)
            }
        }
        Err(e) => println!("An error occurred: {:?}", e),
    }

    // set Cargo's linker path:
    println!("cargo:rustc-link-search=target/debug/");

    // link the two libraries we just created to our Rust application:
    println!("cargo:rustc-link-lib=dylib=cdio-wrapper");
    println!("cargo:rustc-link-lib=dylib=cddb-wrapper");
}
