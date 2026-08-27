---
type: api
title: "C API Reference"
description: "The C binding for Honzo via Diplomat"
source: "https://nisoku.org/Honzo/api/c/"
path: /api/c/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:29:06.896Z"
---
---
title: "C API Reference"
description: "The C binding for Honzo via Diplomat"
---

The C binding wraps `honzo-core` and `honzo-io` into a plain C API using [Diplomat](https://github.com/rust-diplomat/diplomat). Two modes are provided: the in-memory parser (`HonzoHandle`) and the file-backed streaming reader (`HonzoFileReader`). Building is not yet available.

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

- `HonzoHandle` -- In-memory parser handle (reads entire file into a buffer)
- `HonzoFileReader` -- File-backed streaming reader (reads one chunk at a time)
- `HonzoErrorCode` -- Error codes returned by fallible functions
- `HonzoHead` -- File header struct
- `TocEntry` -- TOC entry struct
- `MetaMap` -- META data as key-value pairs

---

## HonzoFileReader (streaming)

Open a Honzo file from the filesystem and read chunks on demand. Each `get_chunk` call performs one seek, one read, and one LZ4 decompression. The returned data is valid only until the next `get_chunk` call on the same reader.

### HonzoFileReader lifecycle

| Symbol                                  | Signature                                                                      | Returns                                        | Notes                |
|-----------------------------------------|--------------------------------------------------------------------------------|------------------------------------------------|----------------------|
| `HonzoFileReader_open`                  | `DiplomatStringView path, uint16_t reader_version`                             | `HonzoFileReader_open_result`                  | Open from file path. |
| `HonzoFileReader_open_with_private_key` | `DiplomatStringView path, uint16_t reader_version, DiplomatU8View private_key` | `HonzoFileReader_open_with_private_key_result` | Open with DRM key.   |
| `HonzoFileReader_destroy`               | `HonzoFileReader* self`                                                        | `void`                                         | Free the reader.     |

### HonzoFileReader accessors

| Symbol                                         | Signature                                     | Returns                                     | Notes                                                                                                                   |
|------------------------------------------------|-----------------------------------------------|---------------------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| `HonzoFileReader_chunk_count`                  | `const HonzoFileReader* self`                 | `uint32_t`                                  | Number of chunks.                                                                                                       |
| `HonzoFileReader_get_chunk_type`               | `const HonzoFileReader* self, uint32_t index` | `uint32_t`                                  | 4-byte chunk tag as native-endian u32 (e.g. `0x50414843` for `"CHAP"`). Returns 0 if index out of range.                |
| `HonzoFileReader_get_chunk_content_type_kind`  | `const HonzoFileReader* self, uint32_t index` | `uint8_t`                                   | Content type kind: `1` = markup, `2` = math. Returns 0 if index out of range.                                           |
| `HonzoFileReader_get_chunk_content_type_value` | `const HonzoFileReader* self, uint32_t index` | `uint8_t`                                   | Content type value: markup `0` = Markdown, `1` = HTML; math `0` = MathML, `1` = LaTeX. Returns 0 if index out of range. |
| `HonzoFileReader_get_chunk_alt_text`           | `const HonzoFileReader* self, uint32_t index` | `HonzoFileReader_get_chunk_alt_text_result` | Human-readable label: chapter title for CHAP, alt text for IMG_, etc. Empty view if index out of range or no label.     |
| `HonzoFileReader_get_chunk`                    | `HonzoFileReader* self, uint32_t index`       | `HonzoFileReader_get_chunk_result`          | Decompressed chunk data. Valid until next call to this function.                                                        |
| `HonzoFileReader_get_meta`                     | `HonzoFileReader* self, DiplomatWrite* write` | `HonzoFileReader_get_meta_result`           | JSON metadata string.                                                                                                   |

## Metadata extraction without parsing

Extract the META block as JSON directly from a file on disk without loading
chunks into memory. Returns `Ok(())` on success; on failure returns
`HonzoErrorCode_FileNotFound`, `HonzoErrorCode_InvalidMagic`,
`HonzoErrorCode_ReaderVersionTooOld`, or `HonzoErrorCode_Truncated`.

| Symbol                       | Signature                                                                | Returns                             | Notes                 |
|------------------------------|--------------------------------------------------------------------------|-------------------------------------|-----------------------|
| `hzo_extract_meta_from_file` | `DiplomatStringView path, uint16_t reader_version, DiplomatWrite* write` | `hzo_extract_meta_from_file_result` | JSON metadata string. |

```c
hzo_extract_meta_from_file_result res =
    hzo_extract_meta_from_file(book_path, 1, &write);
if (res.is_ok) {
    // write.buf contains the META block as JSON
}
```

## Image format detection

`diplomat_external_guess_image_mime` identifies image formats from raw bytes without linking the full `image` crate, which doesn't compile for embedded targets. Returns `Ok(mime_string)` on recognition or an error for unknown formats.

```c
diplomat_external_guess_image_mime_result res =
    diplomat_external_guess_image_mime(chunk_bytes, &write);
if (res.is_ok) {
    // write.buf contains e.g. "image/png", "image/jpeg", "image/gif", etc.
}
```

Recognizes: PNG, JPEG, GIF, BMP, TIFF (LE/BE), WebP, ICO, PNM (PBM/PGM/PPM/PAM).

### Example

```c
#include "HonzoFileReader.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    DiplomatStringView book_path = {"/sdcard/books/book.hzo", 22};
    HonzoFileReader_open_result r =
        HonzoFileReader_open(book_path, 1);
    if (!r.is_ok) {
        fprintf(stderr, "open failed: error %d\n", r.err);
        return 1;
    }
    HonzoFileReader* reader = r.ok;

    uint32_t n = HonzoFileReader_chunk_count(reader);

    uint32_t covr_tag, covt_tag, img_tag, chap_tag;
    memcpy(&covr_tag, "COVR", 4);
    memcpy(&covt_tag, "COVT", 4);
    memcpy(&img_tag,  "IMG_", 4);
    memcpy(&chap_tag, "CHAP", 4);

    for (uint32_t i = 0; i < n; i++) {
        uint32_t tag = HonzoFileReader_get_chunk_type(reader, i);

        // Identify image chunks by type
        if (tag == covr_tag || tag == covt_tag || tag == img_tag) {
            HonzoFileReader_get_chunk_result c =
                HonzoFileReader_get_chunk(reader, i);
            if (c.is_ok) {
                char mime_buf[32];
                DiplomatWrite mime_write = diplomat_simple_write(mime_buf, sizeof(mime_buf));
                diplomat_external_guess_image_mime_result mr =
                    diplomat_external_guess_image_mime(
                        (DiplomatU8View){c.ok.data, c.ok.len}, &mime_write);
                printf("chunk %u: %s\n", i,
                       mr.is_ok ? mime_buf : "unknown");
            }
            continue;
        }

        // Only process CHAP chunks beyond images
        if (tag != chap_tag) continue;

        uint8_t kind = HonzoFileReader_get_chunk_content_type_kind(reader, i);
        uint8_t val  = HonzoFileReader_get_chunk_content_type_value(reader, i);

        HonzoFileReader_get_chunk_result c =
            HonzoFileReader_get_chunk(reader, i);
        if (!c.is_ok) continue;

        // c.ok.data / c.ok.len -- copy before next get_chunk call
        printf("chunk %u: kind=%u val=%u size=%zu\n",
               i, kind, val, c.ok.len);
    }

    HonzoFileReader_destroy(reader);
    return 0;
}
```

---

## HonzoHandle (in-memory parser)

Parse a Honzo file from a byte buffer in memory.

### HonzoHandle lifecycle

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

---

## HonzoErrorCode

| Code                                     | Meaning                   |
|------------------------------------------|---------------------------|
| `HonzoErrorCode_Ok = 0`                  | Success                   |
| `HonzoErrorCode_InvalidMagic = 1`        | Not a Honzo file          |
| `HonzoErrorCode_ReaderVersionTooOld = 2` | Reader version too low    |
| `HonzoErrorCode_BufferTooShort = 3`      | Unexpected end of data    |
| `HonzoErrorCode_CrcMismatch = 4`         | CRC32 checksum mismatch   |
| `HonzoErrorCode_EncryptedChunk = 5`      | Chunk requires DRM key    |
| `HonzoErrorCode_InvalidMathML = 6`       | Malformed MathML content  |
| `HonzoErrorCode_Truncated = 7`           | Truncated or corrupt data |
| `HonzoErrorCode_InvalidCss = 8`          | Malformed CSS content     |
| `HonzoErrorCode_InvalidSyncCue = 9`      | Invalid sync cue entry    |
| `HonzoErrorCode_FileNotFound = 10`       | File does not exist       |
| `HonzoErrorCode_Unknown = 255`           | Unspecified error         |

---

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

## Feature flags

The `image` feature (default: on) enables cover thumbnail generation and image validation, and is only needed when writing/building Honzo files. The streaming reader path (`HonzoFileReader`) never uses it.

```bash
# Host build (default, includes image support for builder tools)
cargo build -p honzo-c --release

# Embedded build (no image crate, smaller compile)
cargo build -p honzo-c --release --no-default-features
```

Feature propagation: `honzo-c --image --> honzo-io --image --> honzo-chunks --image`.

## Building

### Host

```bash
cargo build -p honzo-c --release
# Produces: target/release/libhonzo_c.a
# Headers:  Build/crates/honzo-c/include/
```

Link with:

```bash
cc -o reader reader.c -L./target/release -lhonzo_c -lpthread -ldl -lm
```

### ESP32

Requires the `xtensa-esp32-espidf` target. Install via [espup](https://github.com/esp-rs/espup).

```bash
# Build staticlib without image processing
cargo +esp build --release -p honzo-c \
  --target xtensa-esp32-espidf \
  -Zbuild-std=std,panic_abort \
  --no-default-features
# Produces: target/xtensa-esp32-espidf/release/libhonzo_c.a
```

Link your C firmware against the staticlib using the ESP-IDF toolchain. The resulting binary occupies roughly **434 KB** total (431 KB text, 2.2 KB RAM) for a minimal reader.

## Related

::: grids
::: grid
::: button "C++ API" ./cpp.md icon:code
:::
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
