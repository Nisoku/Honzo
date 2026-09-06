---
type: concept
title: Building
description: "Building Honzo from source for all targets"
source: "https://nisoku.org/Honzo/contributing/building/"
path: /contributing/building/
updated: 2026-09-06
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-09-06T02:55:26.738Z"
---
---
title: "Building"
description: "Building Honzo from source for all targets"
---

::: callout tip "Prerequisites"

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- Node.js 18+ (for WASM/demo build)
- [just](https://just.systems/) (command runner)
- wasm-pack (for WASM target)

```bash
just setup
```

This installs all toolchain dependencies: Rust targets, wasm-pack, npm packages, and diplomat-tool.

:::

## The just system

Honzo uses `just` as its primary command runner. Each surface (Rust, TypeScript, Demo, Docs) has its own script under `Build/scripts/`, invoked through `just`.

```bash
just              # list all available commands
just setup        # install all dependencies
just check        # run all surface checks
just test         # run all tests
just format       # check Rust formatting
```

Surface commands:

```bash
just rust check        # fmt + lint + test + wasm + no_std
just rust test         # cargo test --workspace
just rust lint         # cargo clippy --workspace
just rust fmt          # cargo fmt --all

just typescript check  # lint + typecheck + test + build
just typescript build  # npm run build (TypeScript adapter)
just typescript wasm   # wasm-pack build

just demo check        # wasm + test + build
just demo build        # wasm + vite build

just docs check        # test + build (docmd)
just docs build        # docmd build
```

## Build individual crates

```bash
cargo build -p honzo-core       # no_std parser (no alloc)
cargo build -p honzo-chunks     # Chunk semantics
cargo build -p honzo-io         # Builder, reader, stream
cargo build -p honzo-cli        # CLI binary
cargo build -p honzo-convert    # Format conversion
cargo build -p honzo-c          # C FFI (requires diplomat)
```

## WASM build

```bash
just typescript wasm
```

Output goes to `Build/adapters/typescript/wasm/` and is automatically synced to `Demo/src/wasm/`.

To run wasm-pack directly:

```bash
cd Build/crates/honzo-wasm
wasm-pack build --target web --out-dir ../../adapters/typescript/wasm
```

## TypeScript adapter

```bash
just typescript build
```

Or directly:

```bash
cd Build/adapters/typescript
npm install
npm run build
```

## Demo

```bash
just demo build      # production build
```

For development:

```bash
cd Demo
npm install
npm run dev
```

## Documentation

```bash
just docs build
# Output in Docs/site/
```

For development:

```bash
cd Docs
npm install
npm run dev
```

## Release build

```bash
cargo build --workspace --release
./target/release/honzo-cli --version
```

## Test

```bash
just test
```

Or directly:

```bash
cargo test --workspace
```

Integration tests are in `Tests/tests/`. Test fixtures are in `Tests/fixtures/`.

## Using the build scripts directly

Each surface script can also be invoked directly without `just`:

```bash
python3 Build/scripts/rust.py check
python3 Build/scripts/typescript.py setup
python3 Build/scripts/demo.py build
python3 Build/scripts/docs.py build
```
