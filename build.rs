// Build script for local-ai
// Handles offline packaging configuration

fn main() {
    println!("cargo:rustc-env=CARGO_CFG_OFFLINE=1");

    // Embed version info
    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", env!("CARGO_PKG_VERSION"));

    // Mark for offline builds
    println!("cargo:rustc-cfg=feature=\"offline_build\"");

    println!("cargo:rerun-if-changed=Cargo.lock");
}
