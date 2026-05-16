# Build Scripts

Per-surface build/check scripts used by the repository `Justfile`.

Use these through `just` from the repository root:

```bash
just rust check
just typescript check
just demo check
just docs check
just check
```

Each script supports subcommands:

```bash
python3 Build/scripts/rust.py check
python3 Build/scripts/typescript.py setup
python3 Build/scripts/demo.py build
```

`honzo_build.py` runs all configured surfaces and is equivalent to `just check`.
