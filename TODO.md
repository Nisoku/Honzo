# TODO

## AAAAA

- [ ] PAGINATION, DUH

## EXTRA Namespaces

- [X] DRM is envelope-only: `DrmEnvelope` struct is a placeholder (`algorithm`, `iv`, `ciphertext`), real format has `encrypted_chunks`, `key_envelope`, `license_url`, `expires_at`
- [X] No actual AES-256-CBC encrypt/decrypt anywhere

## Demo

- [ ] Search UI is kinda bad
- [ ] META inspector panel doesn't exist
- [ ] Reading progress / bookmarks have no demo or API
- [ ] No settings panel (theme, font size, layout mode)
- [ ] No reading progress persistence (localStorage/IndexedDB)
- [ ] No styling/decoration system for chapters

## Tests

- [X] More tests for DRM

## Docs / CI

- [ ] API reference docs don't exist
- [ ] Getting-started guide doesn't exist
- [ ] No auto-copy to update `.hzo` fixtures in Demo
- [ ] Book Maker (online GUI builder) doesn't exist
