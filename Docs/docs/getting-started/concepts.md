---
title: "Core Concepts"
description: "How Honzo works: zero-copy parsing, pull-based streaming, and tail mutability"
---

A few core ideas make Honzo different from other ebook formats.

::: grids
::: grid
::: card "Zero Copy Parsing" icon:zap
Parse the wire format by pointer cast. No allocation, no scanning.
:::
:::
::: grid
::: card "Pull Based Streaming" icon:wind
Decompress one chapter at a time. Memory proportional to the largest chunk.
:::
:::
::: grid
::: card "Per Chunk Compression" icon:compress
Each chunk picks its own algorithm. Text compresses, images do not.
:::
:::
::: grid
::: card "Tail Mutability" icon:edit
META sits at the end. Edit metadata without touching the data section.
:::
:::
:::

## The File Structure

Every Honzo file follows the same layout:

```txt
+------+-----+-------+------------+------+
| HEAD | TOC | DATA  | EXTRA      | META |
|      |     |       | (optional) |      |
+------+-----+-------+------------+------+
```

::: steps

1. ::: tag "magic" 4 bytes `0x484F4E4F` ("HONO")
2. ::: tag "HEAD" 48 bytes. Format version, chunk count, layout mode, section offsets.
3. ::: tag "TOC" Variable. Fixed size 32 byte entries, one per chunk.
4. ::: tag "DATA" Variable. Raw chunk payloads.
5. ::: tag "EXTRA" Variable. Extensible metadata such as annotations, DRM, and sync tracks.
6. ::: tag "META" Variable. Book metadata encoded as MessagePack.

:::

META is deliberately last. Edit the title, tags, or revision without touching DATA.

## Zero Copy Parsing

The wire format is designed for direct casting. You take the file bytes and overlay structs. No parsing, no allocation.

```rust
let data = std::fs::read("book.hzo").unwrap();
let p = HonzoParser::new(&data, 1).unwrap();

// head() is a pointer cast, not a copy
let head = p.head();
```

When you memory map the file, parsing completes in O(1) time with zero heap allocation. This approach works in environments where every allocation counts. Think embedded devices, WASM runtimes, and kernel modules.

## Pull Based Streaming

The TOC tells you exactly where each chunk lives. You never have to read everything at once.

- Parse only the header (48 bytes) to open a file.
- Skip to META to read the title. No chunk data is parsed.
- Decompress chapters one at a time as the reader turns pages.
- Seek to a specific image by its TOC offset.

The streaming API (`HonzoStream`) reads and decompresses one chapter at a time. A 1GB book with 100 chapters uses memory proportional to the largest single chapter.

## Per Chunk Compression

Each TOC entry picks its compression independently:

| Code | Algorithm | Use Case                               |
|------|-----------|----------------------------------------|
| 0    | None      | Images, fonts, already compressed data |
| 1    | LZ4       | Chapter text, CSS, metadata            |

LZ4 favors decode speed over compression ratio. Text chapters typically compress to 40-60 percent of their original size. Since compression is per chunk, you control the tradeoff. Do not waste CPU decompressing JPEG images that will not shrink. Do compress chapters where the savings matter.

## Tail Mutability

META lives at the end of the file. This design lets you:

::: steps

1. Read the file structure by scanning HEAD and TOC.
2. Append a new META block at the end.
3. Update the `meta_offset` and `meta_size` fields in HEAD.

:::

No bytes in DATA need rewriting. The same strategy powers MP4 and other streaming oriented formats. EXTRA entries sit between DATA and META. They are appendable too.

## Chunk Types

| Tag    | Type       | Description                              |
|--------|------------|------------------------------------------|
| `CHAP` | Chapter    | Book chapter content in Markdown or HTML |
| `IMG_` | Image      | Inline image as JPEG, PNG, or WebP       |
| `CSS_` | Stylesheet | CSS for chapter rendering                |
| `FONT` | Font       | Embedded font in WOFF2 or TTF            |
| `COVR` | Cover      | Full size cover image                    |
| `COVT` | Thumbnail  | Cover thumbnail for previews             |
| `NOTE` | Annotation | Embedded annotation data                 |
| `SIDX` | Search     | Inverted search index as MessagePack     |
| `MATH` | Math       | LaTeX or AsciiMath equation              |

See [Chunk Types](../format/chunk-types) for the full reference.

## EXTRA Namespaces

EXTRA entries use reverse domain namespacing for extensibility:

| Namespace         | Purpose                                                       |
|-------------------|---------------------------------------------------------------|
| `org.nisoku.anno` | Portable annotations such as highlights, bookmarks, and notes |
| `org.nisoku.drm`  | AES-256-GCM encryption envelope                               |
| `org.nisoku.sync` | Audio, video, and text synchronization tracks                 |

Anyone can define a custom namespace. Unrecognized entries are preserved during round trips. No data is lost.

## Next Steps

::: grids
::: grid
::: button "Quick Start" ./quickstart.md icon:play
:::
::: grid
::: button "Wire Format" ../format/wire-format.md icon:binary
:::
::: grid
::: button "Streaming" ../features/streaming.md icon:play
:::
:::
