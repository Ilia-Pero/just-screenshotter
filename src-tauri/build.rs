fn main() {
    // Only filming needs these libraries. They are loaded on the first call, so at rest they do
    // not take memory, and the program starts in Windows N, which comes without them.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        for library in ["mfplat", "mfreadwrite", "d3d11"] {
            println!("cargo:rustc-link-arg-bins=/DELAYLOAD:{library}.dll");
        }
        println!("cargo:rustc-link-arg-bins=delayimp.lib");
    }
    tauri_build::build();
}
