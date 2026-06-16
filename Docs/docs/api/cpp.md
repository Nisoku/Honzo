---
title: "C++ API Reference"
description: "The C++ binding for Honzo via Diplomat"
---

Header-only C++ RAII layer over [Diplomat](https://github.com/rust-diplomat/diplomat). All methods are `inline`. Include headers, link `libhonzo_c.a`.

## Usage

```cpp
#include <cpp/HonzoFileReader.hpp>

HonzoFileReader_open_result r = HonzoFileReader::open("/sdcard/books/book.hzo", 1);
if (!r.is_ok()) return;

auto reader = std::move(r).ok();
uint32_t n = reader->chunk_count();

for (uint32_t i = 0; i < n; i++) {
    uint32_t tag = reader->get_chunk_type(i);
    // ... process chunks ...
}
```

| Return                          | Meaning |
|---------------------------------|---------|
| ::: tag "0" color:#22c55e       | Success |
| ::: tag "nonzero" color:#ef4444 | Error   |

## Exported types

- `HonzoFileReader` - file-backed streaming reader (owns an open FD)
- `HonzoHandle` - in-memory parser handle
- `HonzoBuilderHandle` - builder/writer for creating Honzo files
- `TocEntryOwned` - owned C++ struct with per-chunk TOC metadata
- `HonzoErrorCode` - error code enum
- Free functions (`guess_image_mime`, `guess_font_format`, `validate_css`, `validate_mathml`, `latex_to_mathml`, `render_math`, `normalize_search_term`)

---

## HonzoFileReader (streaming)

Open a Honzo file from the filesystem and read chunks on demand. Each `get_chunk` call performs one seek, one read, and one LZ4 decompression. The returned span is valid only until the next `get_chunk` call on the same reader.

### HonzoFileReader lifecycle

Include: `#include <cpp/HonzoFileReader.hpp>`

| Method                                     | Signature                                                                   | Returns                                                              | Notes               |
|--------------------------------------------|-----------------------------------------------------------------------------|----------------------------------------------------------------------|---------------------|
| `open`                                     | `static (std::string_view path, uint16_t reader_version)`                   | `diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>` | Static factory.     |
| `open_with_private_key`                    | `static (std::string_view path, uint16_t version, span<const uint8_t> key)` | `diplomat::result<std::unique_ptr<HonzoFileReader>, HonzoErrorCode>` | Open with DRM key.  |
| `~HonzoFileReader` (via `operator delete`) |                                                                             |                                                                      | Closes the file FD. |

### HonzoFileReader accessors

| Method                         | Signature                | Returns                                         | Notes                                                                                       |
|--------------------------------|--------------------------|-------------------------------------------------|---------------------------------------------------------------------------------------------|
| `chunk_count`                  | `() const`               | `uint32_t`                                      | Number of chunks.                                                                           |
| `get_chunk_type`               | `(uint32_t index) const` | `uint32_t`                                      | 4-byte tag as native-endian u32 (e.g. `0x50414843` for `"CHAP"`). 0 if OOB.                 |
| `get_chunk_content_type_kind`  | `(uint32_t index) const` | `uint8_t`                                       | 1 = markup, 2 = math. 0 if OOB.                                                             |
| `get_chunk_content_type_value` | `(uint32_t index) const` | `uint8_t`                                       | Markup: 0 = Markdown, 1 = HTML. 0 if OOB.                                                   |
| `get_chunk_alt_text`           | `(uint32_t index) const` | `std::optional<std::string_view>`               | Human-readable label (chapter title, image alt text, etc.). `std::nullopt` if OOB or empty. |
| `get_chunk`                    | `(uint32_t index)`       | `std::optional<diplomat::span<const uint8_t>>`  | Decompressed chunk data. Valid until next call on same reader.                              |
| `get_meta`                     | `()`                     | `diplomat::result<std::string, HonzoErrorCode>` | JSON metadata string.                                                                       |

---

## HonzoHandle (in-memory parser)

Parse a Honzo file from a byte buffer in memory. Returns `nullptr` (via `std::unique_ptr`) on error.

### HonzoHandle lifecycle

Include: `#include <cpp/HonzoHandle.hpp>`

| Method                   | Signature                                                                      | Returns                        | Notes                       |
|--------------------------|--------------------------------------------------------------------------------|--------------------------------|-----------------------------|
| `parse`                  | `static (span<const uint8_t> data, uint16_t reader_version)`                   | `std::unique_ptr<HonzoHandle>` | Returns `nullptr` on error. |
| `parse_with_private_key` | `static (span<const uint8_t> data, uint16_t version, span<const uint8_t> key)` | `std::unique_ptr<HonzoHandle>` | Parse with DRM key.         |
| `~HonzoHandle`           |                                                                                |                                | Frees the handle.           |

### HonzoHandle accessors

| Method               | Signature          | Returns                                         | Notes                            |
|----------------------|--------------------|-------------------------------------------------|----------------------------------|
| `chunk_count`        | `() const`         | `uint32_t`                                      | Number of chunks.                |
| `version_major`      | `() const`         | `uint8_t`                                       | File format major version.       |
| `version_minor`      | `() const`         | `uint8_t`                                       | File format minor version.       |
| `min_reader_version` | `() const`         | `uint16_t`                                      | Minimum reader version required. |
| `flags`              | `() const`         | `uint32_t`                                      | Header flags.                    |
| `toc_size`           | `() const`         | `uint64_t`                                      | Size of TOC section in bytes.    |
| `data_size`          | `() const`         | `uint64_t`                                      | Size of DATA section in bytes.   |
| `extra_size`         | `() const`         | `uint64_t`                                      | Size of EXTRA section in bytes.  |
| `meta_size`          | `() const`         | `uint64_t`                                      | Size of META section in bytes.   |
| `layout_mode`        | `() const`         | `uint8_t`                                       | Layout mode from header.         |
| `has_drm`            | `() const`         | `bool`                                          | Whether file has DRM.            |
| `has_sidx`           | `() const`         | `bool`                                          | Whether file has SIDX.           |
| `has_annotations`    | `() const`         | `bool`                                          | Whether file has annotations.    |
| `has_sync`           | `() const`         | `bool`                                          | Whether file has sync cues.      |
| `get_extra`          | `() const`         | `diplomat::span<const uint8_t>`                 | Raw EXTRA section bytes.         |
| `get_chunk`          | `(uint32_t index)` | `std::optional<diplomat::span<const uint8_t>>`  | Decompressed chunk data.         |
| `get_meta`           | `() const`         | `diplomat::span<const uint8_t>`                 | Raw META MessagePack bytes.      |
| `get_meta_parsed`    | `() const`         | `diplomat::result<std::string, HonzoErrorCode>` | JSON metadata string.            |
| `get_annotations`    | `() const`         | `diplomat::result<std::string, HonzoErrorCode>` | JSON annotations string.         |
| `get_sync_cues`      | `() const`         | `diplomat::result<std::string, HonzoErrorCode>` | JSON sync cues string.           |
| `get_pmap`           | `() const`         | `diplomat::result<std::string, HonzoErrorCode>` | JSON page map string.            |
| `get_toc`            | `() const`         | `diplomat::result<std::string, HonzoErrorCode>` | JSON TOC string.                 |

Methods ending in `_write(W&)` overloads are also available for writing directly to a custom writeable (e.g. a fixed buffer).

---

## HonzoBuilderHandle (builder)

Construct Honzo files programmatically. Add chunks, set metadata, configure DRM, then `finalize()` and read the result.

### HonzoBuilderHandle lifecycle

Include: `#include <cpp/HonzoBuilderHandle.hpp>`

| Method                | Signature   | Returns                               | Notes                    |
|-----------------------|-------------|---------------------------------------|--------------------------|
| `new_`                | `static ()` | `std::unique_ptr<HonzoBuilderHandle>` | Create empty builder.    |
| `finalize`            | `()`        | `bool`                                | Finalize the file.       |
| `get_result`          | `() const`  | `diplomat::span<const uint8_t>`       | Finalized Honzo bytes.   |
| `~HonzoBuilderHandle` |             |                                       | Frees builder resources. |

### HonzoBuilderHandle setters

| Method                   | Signature                                                                                                                 | Returns | Notes                                     |
|--------------------------|---------------------------------------------------------------------------------------------------------------------------|---------|-------------------------------------------|
| `add_chunk`              | `(tag, data, compression, content_type_kind, content_type_value, cover_type, alt_text, font_embedding, font_license_url)` | `bool`  | Add a data chunk.                         |
| `set_language`           | `(std::string_view lang)`                                                                                                 | `bool`  | Set book language (ISO 639-1).            |
| `set_auto_sidx`          | `(bool enable)`                                                                                                           | `bool`  | Auto-generate SIDX on finalize.           |
| `set_auto_covt`          | `(bool enable)`                                                                                                           | `bool`  | Auto-generate COVT from COVR on finalize. |
| `set_layout`             | `(uint8_t layout)`                                                                                                        | `bool`  | Set layout mode.                          |
| `set_flags`              | `(uint32_t flags)`                                                                                                        | `bool`  | Set header flags.                         |
| `set_min_reader_version` | `(uint16_t version)`                                                                                                      | `bool`  | Set minimum reader version.               |
| `add_pmap_entry`         | `(uint32_t print_page, uint32_t chunk_id, uint32_t byte_offset)`                                                          | `bool`  | Add page map entry.                       |
| `add_math_chunk`         | `(span<const uint8_t> data, uint8_t math_type, uint8_t compression)`                                                      | `bool`  | Add a math chunk.                         |
| `set_meta`               | `(span<const uint8_t> msgpack)`                                                                                           | `bool`  | Set raw MessagePack metadata.             |
| `set_extra`              | `(span<const uint8_t> extra)`                                                                                             | `bool`  | Set raw EXTRA section bytes.              |
| `add_extra_entry`        | `(tag, std::string_view namespace_, span<const uint8_t> body)`                                                            | `bool`  | Add an EXTRA entry.                       |
| `add_annotation`         | `(span<const uint8_t> body)`                                                                                              | `bool`  | Add an annotation body.                   |
| `set_drm_config`         | `(encrypt_chunk_ids, recipient_public_key, license_url, expires_at)`                                                      | `bool`  | Configure DRM encryption.                 |
| `add_sync_cue`           | `(span<const uint8_t> body)`                                                                                              | `bool`  | Add a sync cue entry.                     |

Parameters follow the same semantics as the [Rust builder](./rust.md#honzobuilder).

---

## TocEntryOwned struct

Include: `#include <cpp/TocEntryOwned.hpp>`

```cpp
struct TocEntryOwned {
    uint32_t chunk_id;
    uint64_t offset;
    uint32_t size_compressed;
    uint32_t size_raw;
    uint8_t  compression;
    uint8_t  ctype_kind;
    uint8_t  ctype_value;
    uint8_t  cover_type;
    uint8_t  flags;
    uint32_t crc32;
};
```

Owned (non-opaque) C++ struct, safe to copy.

---

## HonzoErrorCode

Include: `#include <cpp/HonzoErrorCode.hpp>`

| Value                                     | Meaning                   |
|-------------------------------------------|---------------------------|
| `HonzoErrorCode::Ok = 0`                  | Success                   |
| `HonzoErrorCode::InvalidMagic = 1`        | Not a Honzo file          |
| `HonzoErrorCode::ReaderVersionTooOld = 2` | Reader version too low    |
| `HonzoErrorCode::BufferTooShort = 3`      | Unexpected end of data    |
| `HonzoErrorCode::CrcMismatch = 4`         | CRC32 checksum mismatch   |
| `HonzoErrorCode::EncryptedChunk = 5`      | Chunk requires DRM key    |
| `HonzoErrorCode::InvalidMathML = 6`       | Malformed MathML content  |
| `HonzoErrorCode::Truncated = 7`           | Truncated or corrupt data |
| `HonzoErrorCode::InvalidCss = 8`          | Malformed CSS content     |
| `HonzoErrorCode::InvalidSyncCue = 9`      | Invalid sync cue entry    |
| `HonzoErrorCode::FileNotFound = 10`       | File does not exist       |
| `HonzoErrorCode::Unknown = 255`           | Unspecified error         |

---

## Free functions

Include: `#include <cpp/free_functions.hpp>`

| Function                | Signature                                        | Returns                                         | Notes                               |
|-------------------------|--------------------------------------------------|-------------------------------------------------|-------------------------------------|
| `guess_image_mime`      | `(span<const uint8_t> bytes)`                    | `diplomat::result<std::string, HonzoErrorCode>` | Detect image format from raw bytes. |
| `guess_font_format`     | `(span<const uint8_t> bytes)`                    | `diplomat::result<std::string, HonzoErrorCode>` | Detect font format.                 |
| `latex_to_mathml`       | `(span<const uint8_t> bytes)`                    | `diplomat::result<std::string, HonzoErrorCode>` | Convert LaTeX to MathML.            |
| `render_math`           | `(span<const uint8_t> bytes, uint8_t math_type)` | `diplomat::result<std::string, HonzoErrorCode>` | Render math expression.             |
| `normalize_search_term` | `(std::string_view term, std::string_view lang)` | `diplomat::result<std::string, HonzoErrorCode>` | Normalize text for search indexing. |
| `validate_css`          | `(span<const uint8_t> bytes)`                    | `bool`                                          | Validate CSS syntax.                |
| `validate_mathml`       | `(span<const uint8_t> bytes)`                    | `bool`                                          | Validate MathML syntax.             |

Each function also has a `_write(W&)` overload for custom writeable targets.

### Image format detection example

```cpp
#include <cpp/HonzoFileReader.hpp>
#include <cpp/free_functions.hpp>

auto r = HonzoFileReader::open("/sdcard/books/book.hzo", 1);
if (!r.is_ok()) return;
auto reader = std::move(r).ok();

auto chunk = reader->get_chunk(0);
if (chunk.has_value()) {
    auto mime = guess_image_mime(chunk.value());
    if (mime.is_ok()) {
        // mime.ok() == "image/png", "image/jpeg", etc.
    }
}
```

Recognizes: PNG, JPEG, GIF, BMP, TIFF (LE/BE), WebP, ICO, PNM (PBM/PGM/PPM/PAM).

---

## Building

### Host

```bash
cargo build -p honzo-c --release
# Produces: target/release/libhonzo_c.a
# Headers:  Build/crates/honzo-c/include/cpp/
```

Link your C++ program against the static library:

```bash
c++ -o reader reader.cpp -L./target/release -lhonzo_c -lpthread -ldl -lm
```

### ESP32

Requires the `xtensa-esp32-espidf` target (install via [espup](https://github.com/esp-rs/espup)).

```bash
# Build staticlib without image processing
cargo +esp build --release -p honzo-c \
  --target xtensa-esp32-espidf \
  -Zbuild-std=std,panic_abort \
  --no-default-features

# Link firmware (PlatformIO extra_script.py):
#   env.Prepend(LIBPATH=["lib/honzo_c/xtensa-esp32s3-espidf/release"])
#   env.Prepend(LIBS=["honzo_c"])
```

Add to `platformio.ini`:

```ini
build_flags =
    -Ithird_party/honzo/Build/crates/honzo-c/include
lib_deps =
    # ... add the .a via extra_script.py
```

### Feature flags

The `image` feature (default: on) enables cover thumbnail generation and is only needed when building files. The streaming reader path (`HonzoFileReader`) never uses it.

```bash
# Embedded build (no image crate, smaller compile)
cargo build -p honzo-c --release --no-default-features
```

## Related

::: grids
::: grid
::: button "C API" ./c.md icon:code
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
