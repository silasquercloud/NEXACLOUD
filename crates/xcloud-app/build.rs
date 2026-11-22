//! Windows: embeds the app icon, version information and the application manifest
//! (`packaging/windows/nexacloud.manifest`) into `nexacloud.exe`. Nothing to do on
//! other targets.

fn main() {
    #[cfg(windows)]
    windows_resources();
}

#[cfg(windows)]
fn windows_resources() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    const ICON: &str = "../../assets/icons/nexacloud.ico";
    const MANIFEST: &str = "../../packaging/windows/nexacloud.manifest";
    println!("cargo:rerun-if-changed={ICON}");
    println!("cargo:rerun-if-changed={MANIFEST}");
    let mut resource = winresource::WindowsResource::new();
    // FileVersion / ProductVersion come from CARGO_PKG_VERSION.
    resource
        .set_icon(ICON)
        .set_manifest_file(MANIFEST)
        .set("ProductName", "NexaCloud")
        .set("FileDescription", "NexaCloud")
        .set("OriginalFilename", "nexacloud.exe")
        .set("InternalName", "nexacloud")
        .set("LegalCopyright", "NexaCloud contributors, GPL-3.0-only");
    resource
        .compile()
        .expect("compile Windows resources (needs the Windows SDK's rc.exe)");
}
