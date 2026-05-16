#!/usr/bin/env python3
"""Rust workspace commands for Honzo."""

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CMDS = {
    "setup": ["rustup", "target", "add", "wasm32-unknown-unknown"],
    "test": ["cargo", "test", "--workspace"],
    "fmt": ["cargo", "fmt", "--all"],
    "lint": ["cargo", "clippy", "--workspace", "--", "-D", "warnings"],
}


def run(cmd: list[str], cwd: Path = ROOT) -> int:
    return subprocess.run(cmd, cwd=cwd).returncode


def fmt_check() -> int:
    result = subprocess.run(
        ["cargo", "fmt", "--all", "--", "--check"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if result.returncode == 0:
        return 0
    print(result.stdout, result.stderr, sep="", end="", flush=True)
    subprocess.run(["cargo", "fmt", "--all"], cwd=ROOT)
    print("[WARN] Rust format issues auto-fixed. Stage changes before committing.", flush=True)
    return result.returncode


def wasm_check() -> int:
    if shutil.which("wasm-pack") is None:
        print("[WARN] wasm-pack not installed; skipping wasm check.", flush=True)
        return 0
    return run(
        [
            "wasm-pack",
            "build",
            "--target",
            "bundler",
            "--out-dir",
            "../../adapters/typescript/wasm",
        ],
        cwd=ROOT / "Build" / "crates" / "honzo-wasm",
    )


def main() -> None:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"

    if cmd == "fmt_check":
        sys.exit(fmt_check())
    elif cmd == "check":
        sys.exit(sum([fmt_check(), run(CMDS["lint"]), run(CMDS["test"]), wasm_check()]))
    elif cmd == "wasm_check":
        sys.exit(wasm_check())
    elif cmd in CMDS:
        sys.exit(run(CMDS[cmd]))
    else:
        print(f"Usage: {sys.argv[0]} <check|fmt_check|wasm_check|{'|'.join(CMDS)}>")
        sys.exit(1)


if __name__ == "__main__":
    main()
