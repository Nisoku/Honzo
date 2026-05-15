fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=diplomat.toml");
    std::fs::create_dir_all("include").ok();
    match std::process::Command::new("diplomat-tool")
        .args(["c", "."])
        .output()
    {
        Ok(output) if output.status.success() => {
            let headers = ["HonzoHandle.h", "HonzoHandle.d.h", "HonzoBuilderHandle.h", "HonzoBuilderHandle.d.h", "diplomat_runtime.h"];
            for h in &headers {
                if std::path::Path::new(h).exists() {
                    std::fs::copy(h, format!("include/{}", h)).ok();
                    std::fs::remove_file(h).ok();
                }
            }
        }
        Ok(output) => {
            println!("diplomat-tool: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!("diplomat-tool not found (install with: cargo install diplomat-tool): {}", e);
        }
    }
}
