---
type: concept
title: Contributing
description: "How to contribute to the Honzo project"
source: "https://nisoku.org/Honzo/contributing/"
path: /contributing/
updated: 2026-09-06
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-09-06T02:54:25.712Z"
---
---
title: "Contributing"
description: "How to contribute to the Honzo project"
---

Honzo is open source under Apache 2.0. Contributions are welcome. You can submit bug reports, feature requests, documentation improvements, and code changes.

## Project links

- [GitHub](https://github.com/Nisoku/Honzo)
- [Issue tracker](https://github.com/Nisoku/Honzo/issues)
- [Discussions](https://github.com/Nisoku/Honzo/discussions)

## Quick start

Prerequisites: [just](https://just.systems/) (command runner), Rust 1.75+, Node.js 18+.

```bash
git clone https://github.com/Nisoku/Honzo
cd Honzo
just setup
just check
```

## What to work on

- **Bug fixes.** Check the issue tracker for tagged issues.
- **Format extensions.** New EXTRA namespaces, chunk types, or compression algorithms.
- **New adapters.** Python, Java, Go, and Swift bindings are all welcome.
- **Documentation.** This site is in `Docs/`, built with docmd.
- **Demo improvements.** The demo apps are in `Demo/`.

## Guidelines

Before submitting a PR, run all surface checks:

```bash
just check
```

This runs Rust formatting, linting, testing, WASM no_std verification, TypeScript typechecking, demo build, and docs build in sequence.

To run individual checks instead:

```bash
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --check
```

Update CHANGELOG.md with notable changes before submitting.

## Pages in this section

::: grids
::: grid
::: button "Building" ./building.md icon:wrench
:::
::: grid
::: button "Architecture" ./architecture.md icon:git-merge
:::
:::
