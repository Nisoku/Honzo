---
type: api
title: "Rust API Reference"
description: "The Rust API for parsing, building, and streaming Honzo files"
source: "https://nisoku.org/Honzo/api/rust/"
path: /api/rust/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:38:36.582Z"
---
---
title: "Rust API Reference"
description: "The Rust API for parsing, building, and streaming Honzo files"
---

The Rust API is the primary surface for Honzo. It is organized across two crates. Use `honzo-core` for no_std parsing. Use `honzo-io` for everything else.

## honzo-core (no_std)

::: callout tip "Zero-dependency"
`honzo-core` has no dependencies. Use it in embedded contexts, kernels, and audit-critical paths where you cannot allocate.
:::

| Symbol                 | Signature                                      | Returns         | Notes                                      |
| ---------------------- | ---------------------------------------------- | --------------- | ------------------------------------------ |
| `HonzoParser::new`     | `pub fn new(data: &[u8], version: u32)`        | `Result<Self>`  | Parse HEAD from byte slice. No alloc.      |
| `parser.head`          | `pub fn head(&self) -> &HonzoHead`             | `&HonzoHead`    | Pointer-cast to HEAD struct.               |
| `parser.toc_entries`   | `pub fn toc_entries(&self) -> &[TocEntry]`     | `&[TocEntry]`   | Pointer-cast to TOC array.                 |
| `parser.read_chunk`    | `pub fn read_chunk(&self, entry: &TocEntry)`   | `Result<&[u8]>` | Returns raw chunk bytes. No decompression. |
| `parser.extra_entries` | `pub fn extra_entries(&self) -> &[ExtraEntry]` | `&[ExtraEntry]` | EXTRA entry references.                    |
| `parser.read_extra`    | `pub fn read_extra(&self, entry: &ExtraEntry)` | `Result<&[u8]>` | Raw extra entry bytes.                     |
| `parser.meta`          | `pub fn meta(&self) -> &[u8]`                  | `&[u8]`         | Raw META bytes (MessagePack).              |

## honzo-io

Full featured crate with Builder, streaming reader, and compression.

### Main types

| Type          | Purpose                                        |
| ------------- | ---------------------------------------------- |
| `HonzoParser` | Zero-copy parser (re-exported from honzo-core) |
| `HonzoStream` | Pull-based streaming reader                    |
| `HonzoReader` | Reader with DRM decryption support             |
| `Builder`     | Programmatic file builder                      |
| `DrmConfig`   | DRM configuration                              |

### Reader API

| Symbol                | Signature                                    | Returns           | Notes                                                 |
| --------------------- | -------------------------------------------- | ----------------- | ----------------------------------------------------- |
| `HonzoReader::new`    | `pub fn new(data: &[u8], version: u32)`      | `Self`            | Create reader.                                        |
| `reader.with_drm_key` | `pub fn with_drm_key(self, key: &[u8; 32])`  | `Self`            | Set X25519 private key for DRM.                       |
| `reader.parse`        | `pub fn parse(&mut self)`                    | `Result<()>`      | Parse file structure and unwrap DRM envelope.         |
| `reader.head`         | `pub fn head(&self) -> &HonzoHead`           | `&HonzoHead`      | File header.                                          |
| `reader.toc_entries`  | `pub fn toc_entries(&self) -> &[TocEntry]`   | `&[TocEntry]`     | Table of contents.                                    |
| `reader.read_chunk`   | `pub fn read_chunk(&self, entry: &TocEntry)` | `Result<Vec<u8>>` | Read and decompress a chunk. Decrypts if DRM key set. |

### Stream API

| Symbol                | Signature                                   | Returns               | Notes                         |
| --------------------- | ------------------------------------------- | --------------------- | ----------------------------- |
| `HonzoStream::open`   | `pub fn open(file: File, version: u32)`     | `Result<Self>`        | Open from std::fs::File.      |
| `stream.with_drm_key` | `pub fn with_drm_key(self, key: &[u8; 32])` | `Self`                | Enable DRM decryption.        |
| `stream.head`         | `pub fn head(&self) -> &HonzoHead`          | `&HonzoHead`          | File header.                  |
| `stream.chapters`     | `pub fn chapters(&mut self)`                | `ChapterIter`         | Iterator over chapter chunks. |
| `stream.chunk`        | `pub fn chunk(&mut self, idx: usize)`       | `Result<ChapterData>` | Read specific chunk by index. |

### Builder API

| Symbol                 | Signature                                                                                       | Returns           | Notes                       |
| ---------------------- | ----------------------------------------------------------------------------------------------- | ----------------- | --------------------------- |
| `Builder::new`         | `pub fn new()`                                                                                  | `Self`            | New builder.                |
| `builder.meta_title`   | `pub fn meta_title(self, lang, title)`                                                          | `Self`            | Set localized title.        |
| `builder.meta_creator` | `pub fn meta_creator(self, lang, creator)`                                                      | `Self`            | Set localized creator.      |
| `builder.add_chapter`  | `pub fn add_chapter(self, content, compression, markup)`                                        | `Self`            | Add a chapter chunk.        |
| `builder.add_chunk`    | `pub fn add_chunk(self, tag, data, compression, markup, cover, lang, font_embed, font_license)` | `Self`            | Add any chunk type.         |
| `builder.with_drm`     | `pub fn with_drm(self, config: DrmConfig)`                                                      | `Self`            | Enable DRM encryption.      |
| `builder.finalize`     | `pub fn finalize(self)`                                                                         | `Result<Vec<u8>>` | Build the Honzo file bytes. |

### Crypto API

| Symbol            | Signature                                  | Returns           | Notes                        |
| ----------------- | ------------------------------------------ | ----------------- | ---------------------------- |
| `generate_cek`    | `pub fn generate_cek()`                    | `[u8; 32]`        | Random 256-bit CEK.          |
| `generate_nonce`  | `pub fn generate_nonce()`                  | `[u8; 12]`        | Random 12-byte nonce.        |
| `encrypt_content` | `pub fn encrypt_content(data, key)`        | `Result<Vec<u8>>` | AES-256-GCM encrypt.         |
| `decrypt_content` | `pub fn decrypt_content(data, key)`        | `Result<Vec<u8>>` | AES-256-GCM decrypt.         |
| `wrap_cek`        | `pub fn wrap_cek(cek, recipient_pub)`      | `Result<Vec<u8>>` | Wrap CEK for recipient.      |
| `unwrap_cek`      | `pub fn unwrap_cek(envelope, private_key)` | `Result<Vec<u8>>` | Unwrap CEK with private key. |

### DrmConfig API

| Symbol                  | Signature                                 | Returns | Notes                                 |
| ----------------------- | ----------------------------------------- | ------- | ------------------------------------- |
| `DrmConfig::new`        | `pub fn new()`                            | `Self`  | New DRM config.                       |
| `config.encrypt_chunks` | `pub fn encrypt_chunks(self, indices)`    | `Self`  | Chunk IDs to encrypt.                 |
| `config.add_recipient`  | `pub fn add_recipient(self, pub_key, id)` | `Self`  | Add a recipient by X25519 public key. |
| `config.expiry`         | `pub fn expiry(self, expiry)`             | `Self`  | Optional expiry timestamp.            |
| `config.license_url`    | `pub fn license_url(self, url)`           | `Self`  | Optional license URL.                 |

## Error model

All fallible operations return `Result<T, HonzoError>`. Error variants include:

| Variant                                  | Meaning                                               |
| ---------------------------------------- | ----------------------------------------------------- |
| ::: tag "InvalidMagic" color:#ef4444     | Not a Honzo file                                      |
| ::: tag "InvalidVersion" color:#ef4444   | Unsupported format version                            |
| ::: tag "InvalidChecksum" color:#ef4444  | Integrity check failed                                |
| ::: tag "CompressionError" color:#ef4444 | LZ4 decompression failure                             |
| ::: tag "DrmError" color:#ef4444         | DRM-related failure. No key, bad envelope, and so on. |
| ::: tag "CryptoError" color:#ef4444      | Encryption or decryption failure                      |

## Related

::: grids
::: grid
::: button "WASM / TypeScript API" ./wasm.md icon:code
:::
::: grid
::: button "C API" ./c.md icon:code
:::
::: grid
::: button "Wire Format" ../format/wire-format.md icon:binary
:::
:::
