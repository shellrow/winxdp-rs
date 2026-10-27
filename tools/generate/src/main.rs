use std::{env, path::PathBuf};
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/winxdp-sys");
    let tool =
        find_msvc_tools::find_tool("x86_64-pc-windows-msvc", "cl.exe").expect("MSVC toolchain");
    let mut b = bindgen::Builder::default()
        .header(root.join("native/bridge.h").to_str().unwrap())
        .clang_args([
            "-x",
            "c",
            "-std=c11",
            "-Wno-everything",
            "--target=x86_64-pc-windows-msvc",
        ])
        .clang_arg(format!("-I{}", root.join("vendor").display()))
        .allowlist_function("wx_.*")
        .allowlist_var("WX_CONST_.*")
        .blocklist_type("HANDLE|OVERLAPPED|_OVERLAPPED|GUID|_GUID")
        .prepend_enum_name(false)
        .derive_default(true)
        .generate_comments(false)
        .layout_tests(true)
        .merge_extern_blocks(true);
    for (key, value) in tool.env() {
        if key == "INCLUDE" {
            for path in env::split_paths(value).filter(|p| !p.as_os_str().is_empty()) {
                b = b.clang_arg(format!(
                    "-I{}",
                    path.to_string_lossy()
                        .replace("\\", "/")
                        .trim_end_matches("/")
                ));
            }
        }
    }
    for name in std::fs::read_to_string(root.join("../../tools/types.txt"))
        .unwrap()
        .lines()
    {
        b = b.allowlist_type(name);
    }
    for name in std::fs::read_to_string(root.join("../../tools/constants.txt"))
        .unwrap()
        .lines()
    {
        b = b.allowlist_var(name);
    }
    let output = b.generate().expect("generate bindings").to_string();
    let mut output = output
        .replace("pub fn wx_", "pub fn ")
        .replace("WX_CONST_", "");
    let funcs = std::fs::read_to_string(root.join("native/bridge.h")).unwrap();
    for line in funcs.lines().filter(|l| l.contains("wx_")) {
        let name = line.split("wx_").nth(1).unwrap().split('(').next().unwrap();
        output = output.replace(
            &format!("pub fn {name}("),
            &format!("#[link_name = \"wx_{name}\"]\npub fn {name}("),
        );
    }
    output=output.replace("pub fn XdpCreateProgram(", "#[cfg(feature = \"legacy-rules\")]\n#[deprecated(note = \"Use eBPF steering; upstream plans removal\")]\npub fn XdpCreateProgram(");
    std::fs::write(root.join("src/bindings.rs"), output).unwrap();
    assert!(
        std::process::Command::new("rustfmt")
            .args(["--edition", "2024"])
            .arg(root.join("src/bindings.rs"))
            .status()
            .unwrap()
            .success()
    );
}
