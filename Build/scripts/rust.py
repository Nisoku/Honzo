#!/usr/bin/env python3
"""Rust workspace commands for Honzo."""

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPTS = Path(__file__).resolve().parent

NO_STD_TARGET = "aarch64-unknown-none"

CMDS = {
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
    return subprocess.run([sys.executable, str(SCRIPTS / "typescript.py"), "wasm"], cwd=ROOT).returncode


def no_std_check() -> int:
    code = 0
    features = [
        "--no-default-features",
        "--no-default-features --features alloc",
        "--no-default-features --features compression",
        "--no-default-features --features alloc,compression",
    ]
    for feat in features:
        result = subprocess.run(
            ["cargo", "check", "-p", "honzo-core"]
            + feat.split()
            + ["--target", NO_STD_TARGET],
            cwd=ROOT,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print(f"[FAIL] honzo-core {feat} on {NO_STD_TARGET}", flush=True)
            print(result.stdout, result.stderr, sep="", end="", flush=True)
            code = 1
        else:
            print(f"[OK]   honzo-core {feat} on {NO_STD_TARGET}", flush=True)
    return code


def main() -> None:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"

    if cmd == "fmt_check":
        sys.exit(fmt_check())
    elif cmd == "check":
        sys.exit(sum([fmt_check(), run(CMDS["lint"]), run(CMDS["test"]), wasm_check(), no_std_check()]))
    elif cmd == "wasm_check":
        sys.exit(wasm_check())
    elif cmd == "no_std_check":
        sys.exit(no_std_check())
    elif cmd in CMDS:
        sys.exit(run(CMDS[cmd]))
    elif cmd == "setup":
        code = 0
        if not shutil.which("diplomat-tool"):
            code += run(["cargo", "install", "diplomat-tool"])
        if not shutil.which("wasm-pack"):
            code += run(["cargo", "install", "wasm-pack"])
        code += run(["rustup", "target", "add", "wasm32-unknown-unknown", NO_STD_TARGET])
        sys.exit(code)
    else:
        print(f"Usage: {sys.argv[0]} <check|fmt_check|wasm_check|no_std_check|{'|'.join(CMDS)}>")
        sys.exit(1)


if __name__ == "__main__":
    main()
