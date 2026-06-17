//! Build script for tokscale-cli.
//!
//! When (and only when) the optional `apple-fm` feature is enabled AND the
//! target OS is macOS, this builds the vendored `foundation-models-c` SwiftPM
//! package and links the resulting `libFoundationModels.dylib`.
//!
//! When the feature is off, or the target is not macOS, this build script is a
//! complete no-op so that cross-platform / default builds are unaffected.

use std::path::Path;
use std::process::Command;

fn main() {
    // Re-run only when the feature flag toggles. (Cheap; keeps the no-op path no-op.)
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_APPLE_FM");

    let feature_enabled = std::env::var("CARGO_FEATURE_APPLE_FM").is_ok();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // No-op unless the feature is enabled and we're building for macOS.
    if !feature_enabled || target_os != "macos" {
        return;
    }

    build_apple_fm();
}

fn build_apple_fm() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set by cargo");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set by cargo");

    let pkg_dir = Path::new(&manifest_dir).join("vendor/foundation-models-c");
    if !pkg_dir.join("Package.swift").exists() {
        panic!(
            "apple-fm feature is enabled but the vendored SwiftPM package was not found at {}. \
             Expected Package.swift there.",
            pkg_dir.display()
        );
    }

    // Re-run if any vendored Swift source, the manifest, the header, or this
    // build script changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        pkg_dir.join("Package.swift").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        pkg_dir.join("Sources").display()
    );

    // Build the SwiftPM package in release mode.
    let status = Command::new("swift")
        .args(["build", "-c", "release", "--package-path"])
        .arg(&pkg_dir)
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "apple-fm feature is enabled but `swift build` could not be spawned: {e}. \
                 Is the Swift toolchain installed and on PATH?"
            )
        });

    if !status.success() {
        panic!(
            "apple-fm feature is enabled but `swift build -c release` failed in {} \
             (exit status: {status}). Fix the Swift build or disable the apple-fm feature.",
            pkg_dir.display()
        );
    }

    // Locate the produced dylib and copy it into OUT_DIR so the linker (and the
    // runtime rpath below) can find it deterministically.
    let dylib_name = "libFoundationModels.dylib";
    let built_dylib = pkg_dir.join(".build/release").join(dylib_name);
    if !built_dylib.exists() {
        panic!(
            "apple-fm: swift build succeeded but {} was not found",
            built_dylib.display()
        );
    }
    let dest_dylib = Path::new(&out_dir).join(dylib_name);
    std::fs::copy(&built_dylib, &dest_dylib).unwrap_or_else(|e| {
        panic!(
            "apple-fm: failed to copy {} -> {}: {e}",
            built_dylib.display(),
            dest_dylib.display()
        )
    });

    // Link against the dylib in OUT_DIR, and bake an rpath so the final binary
    // can locate it at runtime.
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=dylib=FoundationModels");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir}");
}
