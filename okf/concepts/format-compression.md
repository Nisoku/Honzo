---
type: concept
title: Compression
description: "Per-chunk LZ4 compression in Honzo files"
source: "https://nisoku.org/Honzo/format/compression/"
path: /format/compression/
updated: 2026-09-06
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-09-06T02:53:35.294Z"
---
---
title: "Compression"
description: "Per-chunk LZ4 compression in Honzo files"
---

Honzo applies compression at the chunk level. Each TOC entry selects its own compression algorithm independently of every other chunk.

## Compression Codes

| Code | Algorithm | Typical Use                            |
| ---- | --------- | -------------------------------------- |
| 0    | None      | Images, fonts, already compressed data |
| 1    | LZ4       | Chapter text, CSS, metadata            |

## LZ4

[LZ4](https://lz4.org) is a fast compression algorithm that prioritizes decode speed over compression ratio. Honzo uses it for text heavy chunks where decode speed matters more than squeezing every byte.

Typical compression ratios for chapter text:

| Content                   | Uncompressed | Compressed with LZ4 | Ratio      |
| ------------------------- | ------------ | ------------------- | ---------- |
| Short chapter around 2KB  | 2,048        | About 1,000         | Around 50% |
| Novel chapter around 10KB | 10,240       | About 4,600         | Around 45% |
| CSS file around 50KB      | 51,200       | About 15,000        | Around 29% |

## Per Chunk Tradeoff

Each TOC entry selects its own compression. This lets you optimize per chunk.

Leave images uncompressed. JPEG and WebP are already compressed. Running LZ4 on them wastes CPU and may increase size slightly.

Compress chapters. Text benefits from LZ4. The decode speed is fast enough that page turns feel instant.

Do not compress fonts. WOFF2 is already compressed. TTF and OTF may see modest gains but the overhead rarely matters.

## Size Tracking in the TOC

Each TOC entry tracks two sizes:

| Field       | What It Holds                                                                                |
| ----------- | -------------------------------------------------------------------------------------------- |
| `size`      | Stored size. This is the compressed size or the uncompressed size if no compression applies. |
| `orig_size` | Original uncompressed size. Zero if same as `size`.                                          |

When `compression` equals 1, `size` is the compressed length. `orig_size` is the decompressed length and is non zero.

When `compression` equals 0, `size` equals `orig_size`. Alternatively `orig_size` is 0.

## Implementation

::: tabs

== tab "Rust Builder"

```rust
use honzo_io::{HonzoBuilder, Compression, MarkupType};

HonzoBuilder::new()
    .add_chapter("Chapter 1", Compression::Lz4, MarkupType::Markdown)
    .add_chapter("Chapter 2", Compression::None, MarkupType::HTML)
    .finalize()
```

== tab "Rust Reader"

```rust
use honzo_core::HonzoParser;

let p = HonzoParser::new(&data, 1).unwrap();
for entry in p.toc_entries() {
    let compressed = entry.compression == 1;
    let chunk_data = p.read_chunk(&entry).unwrap();
}
```

:::

## Next Steps

::: grids
::: grid
::: button "Wire Format" ./wire-format.md icon:binary
:::
::: grid
::: button "DRM & Encryption" ./drm.md icon:lock
:::
:::
