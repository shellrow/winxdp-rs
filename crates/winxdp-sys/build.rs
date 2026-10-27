fn main() {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    // docs.rs type-checks Windows APIs on Linux without linking an executable.
    if std::env::var_os("DOCS_RS").is_some() {
        return;
    }
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    cc::Build::new()
        .file("native/bridge.c")
        .file("native/abi.c")
        .file("native/bitfields.c")
        .include("vendor")
        .warnings(false)
        .compile("winxdp_bridge");
    println!("cargo:rustc-link-lib=ntdll");
}
