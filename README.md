# Honzo

[![CI](https://github.com/Nisoku/Honzo/actions/workflows/lint.yml/badge.svg)](https://github.com/Nisoku/Honzo/actions/workflows/lint.yml)
[![npm version](https://img.shields.io/npm/v/@nisoku/honzo.svg)](https://www.npmjs.com/package/@nisoku/honzo)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

---

The "ideal" ebook format.

Honzo is a binary ebook format designed for simplicity, performance, and portability.

## Features

| Feature                      | Description                                                                   |
|------------------------------|-------------------------------------------------------------------------------|
| **Zero-copy parsing**        | Read without allocating: embeddable on bare metal                           |
| **Pull-based streaming**     | Decompress chapters on demand, never hold the whole book in memory            |
| **Per-chunk compression**    | zlib or zstd, selected per chapter via TOC flag                               |
| **Separately editable tail** | META is last: edit title, tags, revision without touching DATA              |
| **Portable annotations**     | org.nisoku.anno in EXTRA: highlights, bookmarks, notes travel with the file |
| **Search index**             | SIDX chunk with inverted term index (MessagePack)                             |
| **Encryption envelope**      | org.nisoku.drm for AES-256-CBC content protection                             |
| **Multi-language metadata**  | Titles and descriptions localized per BCP 47 language tag                     |

## Quick Start

### Rust

```rust
use honzo_std::{HonzoParser, HonzoStream, Builder};

// Zero-copy parse
let data = std::fs::read("book.hzo").unwrap();
let p = HonzoParser::new(&data, 1).unwrap();
println!("{} chunks", p.head().chunk_count);
for entry in p.toc_entries() {
    println!("  {} - {:?}", entry.chunk_id, std::str::from_utf8(&entry.chunk_type));
}

// Streaming read
let file = std::fs::File::open("book.hzo").unwrap();
let mut stream = HonzoStream::open(file, 1).unwrap();
for chapter in stream.chapters() {
    let text = chapter.unwrap();
    println!("Chapter: {} bytes", text.len());
}

// Build
let hzo = Builder::new()
    .add_chunk(*b"CHAP", b"Hello, world!", Compression::None,
               MarkupType::Hmd, CoverType::Front, None, None, None)
    .finalize()
    .unwrap();
```

### TypeScript

```typescript
import { createReader, buildHonzo } from '@nisoku/honzo';

const response = await fetch('book.hzo');
const buf = new Uint8Array(await response.arrayBuffer());
const reader = await createReader(buf);
console.log(reader.chunkCount, reader.layoutMode);

const meta = reader.getMeta();
console.log(meta.title?.en);
```

### C

```c
#include "honzo.h"

HonzoHandle* handle = HonzoHandle_parse(data, data_len, 1);
uint32_t count = HonzoHandle_chunk_count(handle);
```

## Repository Layout

```txt
Honzo/
  Build/
    Cargo.toml              # Rust workspace root
    crates/
      honzo-core/           # no_std core, allocator-optional
      honzo-std/            # std wrapper, compression, streaming
      honzo-convert/        # epub/mobi/pdf import
      honzo-c/              # C FFI bindings (Diplomat)
      honzo-wasm/           # wasm-pack target
      honzo-cli/            # CLI binary
    adapters/typescript/    # npm @nisoku/honzo
  Demo/                     # Vite + TypeScript web demo
  Docs/                     # Documentation site
  Tests/
    fixtures/               # Sample .hzo files
    corpus/                 # Edge case files
    tests/                  # Rust integration tests
  Spec/
    honzo.ksy               # Kaitai Struct spec
```

## Documentation

- [Getting Started](https://nisoku.github.io/Honzo/getting-started/quickstart/)
- [Format Specification](Spec/honzo.ksy)
- [API Reference](https://nisoku.github.io/Honzo/api/)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache License 2.0. See [LICENSE](LICENSE).
