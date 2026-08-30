---
type: concept
title: Installation
description: "Install Honzo on your platform"
source: "https://nisoku.org/Honzo/getting-started/installation/"
path: /getting-started/installation/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T04:03:12.071Z"
---
---
title: "Installation"
description: "Install Honzo on your platform"
---

## Rust

Add `honzo-io` to your `Cargo.toml`:

```toml
[dependencies]
honzo-io = "0.1"
```

The `honzo-io` crate includes the parser, builder, streaming reader, and compression. It requires `std`.

For embedded targets, use `honzo-core` with no_std support:

```toml
[dependencies]
honzo-core = "0.1"
```

This provides the wire format parser without allocation or compression.

## TypeScript / WASM

Install the npm package:

```bash
npm install @nisoku/honzo
```

The package bundles the Honzo WASM binary. It provides `createReader`, `buildHonzo`, and related types for browsers and Node.js.

## C

The C binding ships as a single header library plus a static archive:

```c
#include "honzo.h"
// link against libhonzo_c.a
```

Build from source:

```bash
cargo build -p honzo-c --release
# produces target/release/libhonzo_c.a
```

## CLI

```bash
cargo install honzo-cli
```

Or build from source:

```bash
git clone https://github.com/Nisoku/Honzo
cd Honzo
cargo build -p honzo-cli --release
./target/release/honzo-cli --help
```

### Developer setup

If you plan to contribute or build all surfaces, install [just](https://just.systems/) first:

```bash
git clone https://github.com/Nisoku/Honzo
cd Honzo
cargo install just
just setup     # install Rust targets, npm packages, wasm-pack, diplomat-tool
just check     # verify everything works
```

## Verify

```bash
honzo-cli --version
```

If you see a version number, everything is ready.

## Next Steps

::: grids
::: grid
::: button "Quick Start" ./quickstart.md icon:play
:::
::: grid
::: button "Core Concepts" ./concepts.md icon:book
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
