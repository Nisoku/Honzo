#!/usr/bin/env python3
"""Demo app commands for Honzo."""

import json
import subprocess
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
ROOT = Path(__file__).resolve().parents[2]
DIR = ROOT / "Demo"
PACKAGE_JSON = DIR / "package.json"


def read_scripts() -> dict[str, str]:
    if not PACKAGE_JSON.exists():
        print("[ERROR] Demo package.json not found.", flush=True)
        return {}
    return json.loads(PACKAGE_JSON.read_text()).get("scripts", {})


def run(cmd: list[str]) -> int:
    return subprocess.run(cmd, cwd=DIR).returncode


def run_scripts(cmd: list[str]) -> int:
    return subprocess.run(cmd, cwd=SCRIPTS).returncode


def run_script(name: str) -> int:
    scripts = read_scripts()
    if name not in scripts:
        print(f"[ERROR] demo npm script '{name}' is not defined in {PACKAGE_JSON}.", flush=True)
        return 1
    return run(["npm", "run", name])


def setup() -> int:
    if not PACKAGE_JSON.exists():
        print("[ERROR] Demo package.json not found.", flush=True)
        return 1
    if (DIR / "package-lock.json").exists():
        return run(["npm", "ci"])
    return run(["npm", "install"])


def run_required_scripts(names: list[str]) -> int:
    for name in names:
        code = run_script(name)
        if code != 0:
            return code
    return 0


def ensure_wasm() -> int:
    return run_scripts([sys.executable, str(SCRIPTS / "typescript.py"), "wasm"])


def main() -> None:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"

    if cmd == "setup":
        sys.exit(setup())
    elif cmd == "check":
        if ensure_wasm() != 0:
            sys.exit(1)
        sys.exit(run_required_scripts(["test", "build"]))
    elif cmd == "build":
        if ensure_wasm() != 0:
            sys.exit(1)
        sys.exit(run_script("build"))
    elif cmd == "test":
        if ensure_wasm() != 0:
            sys.exit(1)
        sys.exit(run_script("test"))
    else:
        print(f"Usage: {sys.argv[0]} <check|setup|build|test>")
        sys.exit(1)


if __name__ == "__main__":
    main()
