# TODO

## Format / Spec

- [ ] `min_reader_version` is hardcoded to 1 everywhere, never validated meaningfully

## Chunk Types (empty stub modules)

- [ ] `honzo-chunks/src/data/css.rs`
- [ ] `honzo-chunks/src/data/font.rs`

## EXTRA Namespaces

- [ ] DRM is envelope-only: `DrmEnvelope` struct is a placeholder (`algorithm`, `iv`, `ciphertext`), real format has `encrypted_chunks`, `key_envelope`, `license_url`, `expires_at`
- [ ] No actual AES-256-CBC encrypt/decrypt anywhere
- [ ] `org.nisoku.anno` struct is defined but no programmatic API to build/query annotations
- [ ] `org.nisoku.sync` struct is defined with an explicit `// TODO: Finalize the schema and implement`

## WASM / TypeScript

- [ ] WASM `HonzoWasm` constructor eagerly reads *all* chunks into memory (no streaming)
- [X] WASM API can't pass `alt_text` or `font_embedding` through `honzo_build`
- [ ] C FFI via Diplomat is similarly limited (no alt_text, font_embedding, or extra support on builder)

## Demo

- [ ] Search UI is kinda bad
- [ ] META inspector panel doesn't exist
- [ ] Reading progress / bookmarks have no demo or API
- [ ] No settings panel (theme, font size, layout mode)
- [ ] No reading progress persistence (localStorage/IndexedDB)
- [X] Images don't work properly in chapter rendering
- [ ] No styling/decoration system for chapters
- [ ] Links don't work (<https://nisoku.org/Honzo/demo/endnotes.xhtml#note-209>)

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
