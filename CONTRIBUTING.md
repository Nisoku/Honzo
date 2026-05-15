# Contributing to Honzo

## Getting Started

1. Clone the repo: `git clone https://github.com/Nisoku/Honzo`
2. Build: `cargo build --workspace`
3. Test: `cargo test --workspace`

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- No inline tests. All tests go in `Tests/tests/`
- No unsafe except in the C FFI boundary
- All streaming APIs are pull-based (caller drives)

## Pull Request Process

1. Ensure tests pass: `cargo test --workspace`
2. Run `cargo clippy -- -D warnings`
3. Run `cargo fmt --check`
4. Update CHANGELOG.md with notable changes

## License

By contributing, you agree that your contributions will be licensed under Apache 2.0.
