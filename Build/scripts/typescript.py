#!/usr/bin/env python3
"""TypeScript adapter commands for Honzo."""

import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DIR = ROOT / "Build" / "adapters" / "typescript"
PACKAGE_JSON = DIR / "package.json"
WASM_SOURCE = ROOT / "Build" / "crates" / "honzo-wasm"
WASM_OUT = DIR / "wasm"
DEMO_WASM_OUT = ROOT / "Demo" / "src" / "wasm"


def read_scripts() -> dict[str, str]:
    if not PACKAGE_JSON.exists():
        print("[ERROR] TypeScript adapter package.json not found.", flush=True)
        return {}
    return json.loads(PACKAGE_JSON.read_text()).get("scripts", {})


def run(cmd: list[str]) -> int:
    return subprocess.run(cmd, cwd=DIR).returncode


def run_root(cmd: list[str], cwd: Path = ROOT) -> int:
    return subprocess.run(cmd, cwd=cwd).returncode


def run_script(name: str) -> int:
    scripts = read_scripts()
    if name not in scripts:
        print(f"[ERROR] npm script '{name}' is not defined in {PACKAGE_JSON}.", flush=True)
        return 1
    return run(["npm", "run", name])


def setup() -> int:
    if not PACKAGE_JSON.exists():
        print("[ERROR] TypeScript adapter package.json not found.", flush=True)
        return 1
    if (DIR / "package-lock.json").exists():
        return run(["npm", "ci"])
    return run(["npm", "install"])


def sync_wasm_outputs() -> None:
    if DEMO_WASM_OUT.exists():
        shutil.rmtree(DEMO_WASM_OUT)
    shutil.copytree(WASM_OUT, DEMO_WASM_OUT)


def wasm() -> int:
    if shutil.which("wasm-pack") is None:
        print("[ERROR] wasm-pack is not installed.", flush=True)
        return 1
    if WASM_OUT.exists():
        shutil.rmtree(WASM_OUT)
    result = run_root(
        [
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-dir",
            str(WASM_OUT),
        ],
        cwd=WASM_SOURCE,
    )
    if result != 0:
        return result
    sync_wasm_outputs()
    return 0


def run_required_scripts(names: list[str]) -> int:
    for name in names:
        code = run_script(name)
        if code != 0:
            return code
    return 0


def main() -> None:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"

    if cmd == "setup":
        sys.exit(setup())
    elif cmd == "wasm":
        sys.exit(wasm())
    elif cmd == "check":
        sys.exit(run_required_scripts(["lint", "typecheck", "test", "build"]))
    elif cmd == "test":
        sys.exit(run_script("test"))
    elif cmd == "build":
        sys.exit(run_script("build"))
    elif cmd == "lint":
        sys.exit(run_script("lint"))
    elif cmd == "fmt_check":
        sys.exit(run_script("format"))
    else:
        print(f"Usage: {sys.argv[0]} <check|setup|wasm|build|test|lint|fmt_check>")
        sys.exit(1)


if __name__ == "__main__":
    main()
