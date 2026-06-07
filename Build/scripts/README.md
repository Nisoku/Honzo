# Build Scripts

Per-surface build/check scripts for Honzo.

## Usage

Use `just` from the repository root. This is the primary interface:

```bash
just rust check          # fmt + lint + test + wasm + no_std
just typescript check    # lint + typecheck + test + build
just demo check          # wasm + test + build
just docs check          # test + build
just check               # all surfaces
```

Each script supports subcommands directly too:

```bash
python3 Build/scripts/rust.py check
python3 Build/scripts/typescript.py setup
python3 Build/scripts/demo.py build
python3 Build/scripts/docs.py build
```

`honzo_build.py` runs all configured surfaces and is equivalent to `just check`.
