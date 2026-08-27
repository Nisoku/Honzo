---
type: api
title: "API Reference"
description: "Language-specific API guides for Honzo"
source: "https://nisoku.org/Honzo/api/"
path: /api/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T07:04:26.175Z"
---
---
title: "API Reference"
description: "Language-specific API guides for Honzo"
---

Honzo exposes APIs for four surfaces. Each is a thin layer over the same core format.

## Available APIs

::: grids
::: grid
::: card "Rust" icon:box
Full featured parser, builder, streaming, compression, and DRM support.

::: button "Rust API" ./rust.md icon:code
:::
:::
::: grid
::: card "C++" icon:terminal
RAII wrapper over the C binding. Embedded-friendly, header-only, no STL required.

::: button "C++ API" ./cpp.md icon:code
:::
:::
::: grid
::: card "WASM / TypeScript" icon:cpu
Browser and Node.js support via WebAssembly.

::: button "WASM API" ./wasm.md icon:code
:::
:::
::: grid
::: card "C" icon:terminal
FFI friendly single header binding for read only access.

::: button "C API" ./c.md icon:code
:::
:::
:::

## Surface comparison

| Feature          | Rust                        | C++                         | WASM/TS                     | C                           |
| ---------------- | --------------------------- | --------------------------- | --------------------------- | --------------------------- |
| Parse & inspect  | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e |
| Builder          | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  |
| Streaming reader | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  | ::: tag "Yes" color:#22c55e |
| Compression      | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  |
| DRM              | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  |
| EPUB conversion  | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  | ::: tag "Yes" color:#22c55e | ::: tag "No" color:#ef4444  |
| no_std support   | `honzo-core`                | N/A                         | N/A                         | N/A                         |

::: collapsible "Underlying crates"

All APIs are built on the same Rust workspace:

| Crate           | Description                          | Depends On                                                |
| --------------- | ------------------------------------ | --------------------------------------------------------- |
| `honzo-core`    | no_std wire-format types + parser    | None                                                      |
| `honzo-chunks`  | Chunk semantics (SIDX, COVT, extras) | `honzo-core`                                              |
| `honzo-io`      | Builder, reader, stream, compression | `honzo-core`, `honzo-chunks`                              |
| `honzo-c`       | C FFI via Diplomat                   | `honzo-core`, `honzo-chunks`, `honzo-io`                  |
| `honzo-wasm`    | WASM target                          | `honzo-core`, `honzo-chunks`, `honzo-io`, `honzo-convert` |
| `honzo-cli`     | Command-line binary                  | `honzo-io`, `honzo-convert`                               |
| `honzo-convert` | EPUB/MOBI/PDF import                 | `honzo-io`                                                |

:::
