fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=diplomat.toml");
    println!("cargo:rerun-if-changed=include");
    std::fs::create_dir_all("include").ok();

    let diplomat_bin = std::env::var("CARGO_BIN_EXE_diplomat-tool").unwrap_or_else(|_| {
        std::env::var("DIPLOMAT_TOOL_PATH").unwrap_or_else(|_| "diplomat-tool".into())
    });

    match std::process::Command::new(&diplomat_bin)
        .args(["c", "include/"])
        .output()
    {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            println!(
                "diplomat-tool stderr: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            println!(
                "failed to run diplomat-tool (install with: cargo install diplomat-tool): {}",
                e
            );
        }
    }
}
