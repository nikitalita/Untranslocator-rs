use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=Untranslocator.Lib/build-lib.sh");
    println!("cargo:rerun-if-changed=Untranslocator.Lib/Untranslocator/Untranslocator.h");
    println!("cargo:rerun-if-changed=Untranslocator.Lib/Untranslocator/Untranslocator.m");
    println!("cargo:rerun-if-changed=Untranslocator.Lib/Untranslocator/untranslocator_c.h");
    println!("cargo:rerun-if-changed=Untranslocator.Lib/Untranslocator/untranslocator_c.m");
    println!("cargo:rerun-if-changed=Untranslocator.Lib/Untranslocator.xcodeproj/project.pbxproj");

    if env::var("CARGO_CFG_TARGET_OS").ok().as_deref() != Some("macos") {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let native_dir = manifest_dir.join("Untranslocator.Lib");
    let script_path = native_dir.join("build-lib.sh");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("missing CARGO_CFG_TARGET_ARCH");
    let xcode_arch = match target_arch.as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        other => panic!("unsupported macOS target architecture: {other}"),
    };

    let status = Command::new(&script_path)
        .current_dir(&native_dir)
        .env("CONFIGURATION", "Release")
        .env("UNTRANSLOCATOR_ARCHS", xcode_arch)
        .status()
        .expect("failed to execute native build script");

    if !status.success() {
        panic!("native build failed with status: {status}");
    }

    let native_output_dir = native_dir.join("build").join("output");
    println!("cargo:rustc-link-search=native={}", native_output_dir.display());
    println!("cargo:rustc-link-lib=static=Untranslocator");
    println!("cargo:rustc-link-lib=framework=Cocoa");
}
