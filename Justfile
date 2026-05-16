# Honzo Development Commands
#
#   just              List available commands
#   just rust check   Run Rust checks
#   just check        Run all configured checks

scripts := "Build/scripts"

rust *args:
    cd {{scripts}} && python3 rust.py {{args}}

typescript *args:
    cd {{scripts}} && python3 typescript.py {{args}}

demo *args:
    cd {{scripts}} && python3 demo.py {{args}}

docs *args:
    cd {{scripts}} && python3 docs.py {{args}}

setup:
    cd {{scripts}} && python3 rust.py setup
    cd {{scripts}} && python3 typescript.py setup
    cd {{scripts}} && python3 demo.py setup
    cd {{scripts}} && python3 docs.py setup

format:
    cd {{scripts}} && python3 rust.py fmt_check

test:
    cd {{scripts}} && python3 rust.py test
    cd {{scripts}} && python3 typescript.py test
    cd {{scripts}} && python3 demo.py test
    cd {{scripts}} && python3 docs.py test

check:
    cd {{scripts}} && python3 honzo_build.py

all: setup check
