---
type: concept
title: "Wire Format"
description: "The binary layout of HEAD, TOC, DATA, EXTRA, and META"
source: "https://nisoku.org/Honzo/format/wire-format/"
path: /format/wire-format/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:42:02.797Z"
---
---
title: "Wire Format"
description: "The binary layout of HEAD, TOC, DATA, EXTRA, and META"
---

## HEAD (48 bytes)

The first 4 bytes of HEAD are always the ASCII bytes `HONO` (`0x484F4F4E`). Any file that does not start with these bytes is not a Honzo file.

| Offset | Size | Field              | Description                                                 |
|--------|------|--------------------|-------------------------------------------------------------|
| 0      | 4    | magic              | `"HONO"` magic bytes                                        |
| 4      | 4    | format_version     | Format version, currently `1`                               |
| 8      | 2    | chunk_count        | Number of TOC entries                                       |
| 10     | 1    | layout_mode        | `0` for reflowable, `1` for fixed, `2` for scroll           |
| 11     | 1    | flags              | Bit flags where bit 0 indicates has_extra                   |
| 12     | 8    | data_offset        | Byte offset from file start to DATA section                 |
| 20     | 8    | extra_offset       | Byte offset from file start to EXTRA section. Zero if none. |
| 28     | 8    | meta_offset        | Byte offset from file start to META section                 |
| 36     | 4    | meta_size          | Size of META section in bytes                               |
| 40     | 4    | chunk_table_offset | Byte offset of TOC within the file, relative to DATA start  |
| 44     | 4    | reserved           | Reserved for future use, zero padded                        |

All offsets are absolute from the start of the file.

## TOC (32 bytes x chunk_count)

Each TOC entry is exactly 32 bytes:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
<!-- eslint-disable-next-line markdown/no-missing-label-refs -->
| 0      | 1    | chunk_id[0]          | First byte of the 4-byte chunk type tag                                            |
<!-- eslint-disable-next-line markdown/no-missing-label-refs -->
| 1      | 1    | chunk_id[1]          | Second byte                                                                        |
<!-- eslint-disable-next-line markdown/no-missing-label-refs -->
| 2      | 1    | chunk_id[2]          | Third byte                                                                         |
<!-- eslint-disable-next-line markdown/no-missing-label-refs -->
| 3      | 1    | chunk_id[3]          | Fourth byte, for example `CHAP` or `IMG_`                                          |
| 4      | 1    | content_type         | Content type kind                                                                  |
| 5      | 1    | compression          | `0` for none, `1` for lz4                                                          |
| 6      | 1    | markup_type          | `0` for markdown, `1` for html, applies to CHAP chunks                             |
| 7      | 1    | cover_type           | `0` for none, `1` for front, `2` for back, applies to COVR or COVT                 |
| 8      | 2    | language             | BCP 47 language tag index                                                          |
| 10     | 2    | font_embedding       | Font embedding mode, applies to FONT chunks                                        |
| 12     | 2    | font_license_url_len | Length of font license URL, zero if none                                           |
| 14     | 2    | reserved             | Reserved, zero padded                                                              |
| 16     | 8    | offset               | Byte offset within DATA section                                                    |
| 24     | 4    | size                 | Size of chunk data in bytes. Represents compressed size if compression is nonzero. |
| 28     | 4    | orig_size            | Original uncompressed size. Zero if same as size.                                  |

### Offset and Size Rules

`offset` is relative to `data_offset` from HEAD. The absolute file position of a chunk is `data_offset + entry.offset`.

If `compression` is 0, `size` equals `orig_size`. Alternatively `orig_size` is 0.

If `compression` is 1, `size` represents the compressed length. `orig_size` represents the decompressed length.

## DATA Section

The DATA section starts at `data_offset`. It is a flat array of chunk payloads. Each chunk's position and size are defined by its TOC entry. No separator or framing exists between chunks. The TOC serves as the authoritative index.

```txt
data_offset + 0:     chunk 0 data (entry[0].size bytes)
data_offset + off1:  chunk 1 data (entry[1].size bytes)
data_offset + off2:  chunk 2 data (entry[2].size bytes)
...
```

::: collapsible "EXTRA Section (optional)"

The EXTRA section is variable length and optional. If `flags & 1` equals 0, there is no EXTRA section and `extra_offset` is 0.

```txt
Offset  Size  Field
0       4     extra_count: u32
4       variable  Array of ExtraEntry:
                  - 2 bytes: namespace length (u16)
                  - N bytes: namespace (UTF-8)
                  - 8 bytes: offset (u64, relative to extra_offset)
                  - 4 bytes: size (u32)
                  (entry repeats extra_count times)
```

After the entry array, each entry's data resides at `extra_offset + entry.offset`.

### Standard Namespaces

<!-- markdownlint-disable MD055 -->

| Namespace         | Purpose                                       |
|-------------------|-----------------------------------------------|
| `org.nisoku.anno` | Annotations stored as MessagePack             |
| `org.nisoku.drm`  | DRM envelope stored as MessagePack            |
| `org.nisoku.sync` | Audio/video sync tracks stored as MessagePack |

:::

## META Section

The META section is a MessagePack map. It starts at `meta_offset` and spans `meta_size` bytes. Only `title` and `language` are required. Everything else is optional.

::: collapsible "Example META payload"

```text
{
  "honzo": {
    "version": 1,
    "converter": "honzo-convert 0.1.0",
    "created": "2025-01-15T10:30:00Z",
    "modified": "2025-01-15T10:30:00Z"
  },
  "title": {
    "en": "The Example Book",
    "fr": "Le Livre Exemple"
  },
  "creator": {
    "en": "Author Name"
  },
  "language": "en",
  "description": {
    "en": "A short description of the book."
  },
  "publisher": "Publisher Name",
  "published": "2024-01-01",
  "rights": "Copyright 2024",
  "identifiers": {
    "isbn": "978-0-00-000000-0",
    "doi": "10.0000/example"
  },
  "subject": ["Fiction", "Adventure"],
  "series": "Series Name",
  "series_position": 1,
  "edition": 1,
  "page_progression_direction": "ltr",
  "toc": [
    {"title": "Chapter 1", "src": 0},
    {"title": "Chapter 2", "src": 1}
  ]
}
```

:::

All field names use lowercase with underscores. All text values are maps keyed by BCP 47 language tags. This enables multi language support.

## Next Steps

::: grids
::: grid
::: button "Chunk Types" ./chunk-types.md icon:package
:::
::: grid
::: button "Compression" ./compression.md icon:zap
:::
::: grid
::: button "DRM & Encryption" ./drm.md icon:lock
:::
:::
