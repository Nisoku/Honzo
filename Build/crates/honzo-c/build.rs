fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=diplomat.toml");
    println!("cargo:rerun-if-changed=include");
    std::fs::create_dir_all("include").ok();
    match std::process::Command::new("diplomat-tool")
        .args(["c", "include/"])
        .output()
    {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            println!("diplomat-tool: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!(
                "diplomat-tool not found (install with: cargo install diplomat-tool): {}",
                e
            );
        }
    }
}
