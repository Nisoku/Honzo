---
type: concept
title: Honzo
description: "A binary ebook format designed for simplicity, performance, and portability"
source: "https://nisoku.org/Honzo/"
path: /
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:37:06.094Z"
---
---
title: "Honzo"
description: "A binary ebook format designed for simplicity, performance, and portability"
---

::: hero layout:split glow:true

<!-- markdownlint-disable MD025 -->

# Honzo

A binary ebook format for simplicity, performance, and portability.

::: tag "Zero Copy"
::: tag "Streaming"
::: tag "Portable"

::: button "Quick Start" ./getting-started/quickstart.md icon:play
<!--markdownlint-disable MD034-->

::: button "GitHub" external:https://github.com/Nisoku/Honzo icon:github

== side

::: card "Why Honzo?"
Existing ebook formats are designed for authoring, not consumption. XML parsing, incremental loading, and memory waste on the reader's device.
**Honzo flips this.** Optimized for reading, not writing.
:::
:::

## Features

::: grids
::: grid
::: card "Zero Copy Parsing" icon:zap
Parse without allocating. Memory map the file and read directly from the buffer. This works on bare metal.
:::
:::

::: grid
::: card "Pull Based Streaming" icon:wind
Decompress chapters on demand. Never hold the whole book in memory. Open a 2GB book with zero heap allocation until you turn a page.
:::
:::

::: grid
::: card "Per Chunk Compression" icon:compress
Each TOC entry selects its compression algorithm independently. Chapters with text compress well. Images and fonts stay raw.
:::
:::

::: grid
::: card "Tail Mutability" icon:edit
META sits at the end of the file. Edit the title, tags, or revision without touching the data section.
:::
:::

::: grid
::: card "Portable Annotations" icon:bookmark
Highlights, bookmarks, and notes stored as EXTRA entries under `org.nisoku.anno`. They travel with the file and survive reconversion.
:::
:::

::: grid
::: card "Encryption Envelope" icon:lock
AES-256-GCM content protection via `org.nisoku.drm`. ECDH key exchange with X25519. Per recipient wrapping.
:::
:::
:::

## Quick Example

::: tabs

== tab "Rust"

```rust
use honzo_core::HonzoParser;

let data = std::fs::read("book.hzo").unwrap();
let p = HonzoParser::new(&data, 1).unwrap();

println!("{} chunks", p.head().chunk_count);
for entry in p.toc_entries() {
    println!("  {} - {:?}", entry.chunk_id, std::str::from_utf8(&entry.chunk_type));
}
```

== tab "TypeScript"

```typescript
import { createReader } from "@nisoku/honzo";

const response = await fetch("book.hzo");
const buf = new Uint8Array(await response.arrayBuffer());
const reader = await createReader(buf);

const meta = reader.getMeta();
console.log(meta.title?.en);
```

== tab "C"

```c
#include "honzo.h"

HonzoHandle* handle = HonzoHandle_parse(data, data_len, 1);
uint32_t count = HonzoHandle_chunk_count(handle);
```

:::

## Installation

::: tabs

== tab "Rust"

```toml
[dependencies]
honzo-io = "0.1"
```

== tab "TypeScript"

```bash
npm install @nisoku/honzo
```

== tab "C"

```c
#include "honzo.h"
// link libhonzo_c.a
```

== tab "CLI"

```bash
cargo install honzo-cli
```

:::

## Next Steps

::: grids
::: grid

### Getting Started

Start here if you are new to Honzo.

::: button "Quick Start" ./getting-started/quickstart.md icon:play
::: button "Installation" ./getting-started/installation.md icon:download
:::
::: grid

### Learn the Format

Understand how Honzo works under the hood.

::: button "Core Concepts" ./getting-started/concepts.md icon:book
::: button "Wire Format" ./format/wire-format.md icon:binary
:::
::: grid

### Reference

Browse the full API and CLI documentation.

::: button "CLI Reference" ./cli/ icon:terminal
::: button "API Reference" ./api/ icon:code
:::
:::
