---
title: "C API Reference"
description: "The C binding for Honzo via Diplomat"
---

The C binding wraps `honzo-core` into a plain C API using [Diplomat](https://github.com/rust-diplomat/diplomat). It provides read only access to Honzo files. You can parse, inspect, and read chunks. Building and streaming are not available.

## Usage

```c
#include "honzo.h"

HonzoHandle* handle = HonzoHandle_parse(data, data_len, 1);
uint32_t count = HonzoHandle_chunk_count(handle);
// ... use handle ...
HonzoHandle_destroy(handle);
```

| Return                          | Meaning |
|---------------------------------|---------|
| ::: tag "0" color:#22c55e       | Success |
| ::: tag "nonzero" color:#ef4444 | Error   |

## Exported types

- `HonzoHandle` -- Opaque handle to a parsed Honzo file
- `HonzoHead` -- File header struct
- `TocEntry` -- TOC entry struct
- `MetaMap` -- META data as key-value pairs

## HonzoHandle lifecycle

| Symbol                | Signature                                           | Returns        | Notes                                          |
|-----------------------|-----------------------------------------------------|----------------|------------------------------------------------|
| `HonzoHandle_parse`   | `const uint8_t* data, size_t len, uint32_t version` | `HonzoHandle*` | Parse from byte buffer. Returns NULL on error. |
| `HonzoHandle_destroy` | `HonzoHandle* handle`                               | `void`         | Free the handle.                               |

## HonzoHandle accessors

| Symbol                    | Signature                                                                  | Returns       | Notes                              |
|---------------------------|----------------------------------------------------------------------------|---------------|------------------------------------|
| `HonzoHandle_chunk_count` | `const HonzoHandle*`                                                       | `uint32_t`    | Number of chunks.                  |
| `HonzoHandle_head`        | `const HonzoHandle*, HonzoHead* out`                                       | `int32_t`     | 0 on success, nonzero on error.    |
| `HonzoHandle_toc_entry`   | `const HonzoHandle*, uint32_t index, TocEntry* out`                        | `int32_t`     | 0 on success, nonzero on error.    |
| `HonzoHandle_chunk_data`  | `const HonzoHandle*, uint32_t index, const uint8_t** out, size_t* out_len` | `int32_t`     | Pointer to chunk data. Don't free. |
| `HonzoHandle_meta_raw`    | `const HonzoHandle*, const uint8_t** out, size_t* out_len`                 | `int32_t`     | Raw META MessagePack bytes.        |
| `HonzoHandle_error`       | `const HonzoHandle*`                                                       | `const char*` | Last error message (UTF-8).        |

## HonzoHead struct

```c
typedef struct {
    uint32_t format_version;
    uint16_t chunk_count;
    uint8_t  layout_mode;
    uint8_t  flags;
    uint64_t data_offset;
    uint64_t extra_offset;
    uint64_t meta_offset;
    uint32_t meta_size;
} HonzoHead;
```

## TocEntry struct

```c
typedef struct {
    uint8_t  chunk_type[4];
    uint8_t  content_type;
    uint8_t  compression;
    uint8_t  markup_type;
    uint8_t  cover_type;
    uint16_t language;
    uint16_t font_embedding;
    uint16_t font_license_url_len;
    uint64_t offset;
    uint32_t size;
    uint32_t orig_size;
} TocEntry;
```

## Error handling

Functions return `0` on success and nonzero on error. Get the error message:

```c
const char* msg = HonzoHandle_error(handle);
if (msg) {
    fprintf(stderr, "Honzo error: %s\n", msg);
}
```

## Building

```bash
cargo build -p honzo-c --release
# Produces: target/release/libhonzo_c.a
# Headers:  Build/crates/honzo-c/include/
```

Link with:

```bash
cc -o reader reader.c -L./target/release -lhonzo_c -lpthread -ldl -lm
```

## Related

::: grids
::: grid
::: button "Rust API" ./rust.md icon:code
:::
::: grid
::: button "WASM / TypeScript API" ./wasm.md icon:code
:::
::: grid
::: button "Wire Format" ../format/wire-format.md icon:binary
:::
:::
