#!/usr/bin/env python3
"""Run all configured surface checks for Honzo."""

import subprocess
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
SURFACES = ["rust", "typescript", "demo", "docs"]


def main() -> int:
    failures = 0
    for surface in SURFACES:
        label = f"{surface.capitalize()} check"
        print(f"=== {label} ===", flush=True)
        result = subprocess.run([sys.executable, str(SCRIPTS / f"{surface}.py"), "check"])
        if result.returncode:
            print(f"[FAIL] {label} (exit {result.returncode})", flush=True)
            failures += 1
        else:
            print(f"[PASS] {label}", flush=True)
        print(flush=True)

    if failures:
        print(f"{failures} surface(s) failed", flush=True)
        return 1
    print("All checks passed!", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
