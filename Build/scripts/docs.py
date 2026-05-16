#!/usr/bin/env python3
"""Documentation site commands for Honzo."""

import json
import subprocess
import sys
from pathlib import Path

DIR = Path(__file__).resolve().parents[2] / "Docs"
PACKAGE_JSON = DIR / "package.json"


def read_scripts() -> dict[str, str]:
    if not PACKAGE_JSON.exists():
        print("[ERROR] Docs package.json not found.", flush=True)
        return {}
    return json.loads(PACKAGE_JSON.read_text()).get("scripts", {})


def run(cmd: list[str]) -> int:
    return subprocess.run(cmd, cwd=DIR).returncode


def run_script(name: str) -> int:
    scripts = read_scripts()
    if name not in scripts:
        print(f"[ERROR] docs npm script '{name}' is not defined in {PACKAGE_JSON}.", flush=True)
        return 1
    return run(["npm", "run", name])


def setup() -> int:
    if not PACKAGE_JSON.exists():
        print("[ERROR] Docs package.json not found.", flush=True)
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


def main() -> None:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"

    if cmd == "setup":
        sys.exit(setup())
    elif cmd == "check":
        sys.exit(run_required_scripts(["test", "build"]))
    elif cmd == "build":
        sys.exit(run_script("build"))
    elif cmd == "test":
        sys.exit(run_script("test"))
    else:
        print(f"Usage: {sys.argv[0]} <check|setup|build|test>")
        sys.exit(1)


if __name__ == "__main__":
    main()
