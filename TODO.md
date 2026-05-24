# TODO

## Format / Spec

- [ ] `min_reader_version` is hardcoded to 1 everywhere, never validated meaningfully

## Chunk Types (empty stub modules)

- [ ] `honzo-chunks/src/data/css.rs`
- [ ] `honzo-chunks/src/data/font.rs`
- [ ] `honzo-chunks/src/data/img.rs`

## EXTRA Namespaces

- [ ] DRM is envelope-only: `DrmEnvelope` struct is a placeholder (`algorithm`, `iv`, `ciphertext`), real format has `encrypted_chunks`, `key_envelope`, `license_url`, `expires_at`
- [ ] No actual AES-256-CBC encrypt/decrypt anywhere
- [ ] `org.nisoku.anno` struct is defined but no programmatic API to build/query annotations
- [ ] `org.nisoku.sync` struct is defined with an explicit `// TODO: Finalize the schema and implement`
- [ ] COVT auto-generation not wired into HonzoBuilder, caller must do it manually

## CLI

- [ ] Native converter works but format detection is heuristic and error messages are poor
- [ ] Search command shows chunk IDs but doesn't resolve excerpts (demo does this in JS)
- [x] `honzo tree` command renders .hzo file tree with HEAD/DATA/EXTRA/META sections and type-grouped folders

## WASM / TypeScript

- [X] WASM API is bare-minimum, no typed wrapper, just re-exports wasm-pack output
- [X] TypeScript wrapper has no typed API layer
- [ ] WASM `HonzoWasm` constructor eagerly reads *all* chunks into memory (no streaming)
- [ ] WASM API can't pass `alt_text` or `font_embedding` through `honzo_build`
- [ ] C FFI via Diplomat is similarly limited (no alt_text, font_embedding, or extra support on builder)

## EPUB Conversion

- [x] EPUB conversion logic is duplicated in Rust (`honzo-convert`) and JS (`convert.js` using lexepub)
- [x] Rust `from_epub` strips HTML to text (no image embedding, CSS, fonts in output)
- [x] Rust `from_epub` doesn't preserve chapter structure from nav/toc, only spine order
- [x] Builder auto-generates clean SIDX (HTML stripped) instead of requiring manual build_sidx
- [x] Converter embeds images, CSS, fonts from EPUB manifest into .hzo chunks

## Demo

- [ ] Search UI doesn't exist yet
- [ ] META inspector panel doesn't exist
- [ ] Reading progress / bookmarks have no demo or API
- [ ] No settings panel (theme, font size, layout mode)
- [ ] No reading progress persistence (localStorage/IndexedDB)
- [ ] Images don't work properly in chapter rendering
- [ ] No styling/decoration system for chapters

## Tests

- [ ] More tests needed (round-trip, edge cases, corpus)
- [ ] No tests for CSS_/FONT/IMG_ chunk handling
- [ ] No tests for DRM/anno/sync EXTRA round-trips
- [ ] No tests for WASM or C FFI paths

## Docs / CI

- [ ] API reference docs don't exist
- [ ] Getting-started guide doesn't exist
- [ ] No auto-build to update WASM and `.hzo` fixtures in Demo
- [ ] Book Maker (online GUI builder) doesn't exist

## Housekeeping

- [ ] `honzo-io/src/extra.rs` should validate against the known-namespace registry
- [ ] HonzoStream should preserve alt_text and font_license_url on TOC entry clones
- [ ] NOTE chunks should be included in chapter iteration or exposed as a separate method
