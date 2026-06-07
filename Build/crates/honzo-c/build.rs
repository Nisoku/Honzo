fn main() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo");
    let entry = format!("{}/src/lib.rs", manifest_dir);
    let out_dir = format!("{}/include", manifest_dir);
    let cpp_out_dir = format!("{}/include/cpp", manifest_dir);
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=diplomat.toml");
    println!("cargo:rerun-if-changed=include");
    std::fs::create_dir_all("include").ok();
    std::fs::create_dir_all("include/cpp").ok();

    let diplomat_bin = std::env::var("CARGO_BIN_EXE_diplomat_tool").unwrap_or_else(|_| {
        std::env::var("DIPLOMAT_TOOL_PATH").unwrap_or_else(|_| "diplomat-tool".into())
    });

    for (language, output_dir) in [("c", &out_dir), ("cpp", &cpp_out_dir)] {
        match std::process::Command::new(&diplomat_bin)
            .args([
                language,
                output_dir,
                "--entry",
                &entry,
                "--config-file",
                "diplomat.toml",
            ])
            .output()
        {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                panic!(
                    "diplomat-tool {} failed with status {}\nstdout:\n{}\nstderr:\n{}",
                    language,
                    output.status,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                panic!(
                        "failed to run diplomat-tool {} (install with: cargo install diplomat-tool): {}",
                        language,
                        e
                    );
            }
        }
    }
}
