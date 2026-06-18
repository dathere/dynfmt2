# Conventions

- Edition 2024 (PR #6). The crate migrated cleanly (no source changes from `cargo fix --edition`).
  No `rust-version`/MSRV policy is declared in `Cargo.toml`; the only Rust floor is whatever the
  edition and language features in use (e.g. `let`-chains) require.
- Keep new functionality feature-gated (`#[cfg(feature = ...)]`) and ensure it still
  compiles/tests under `--no-default-features`.
- Cached regexes go through `OnceLock` helpers (`get_python_regex`/`get_curly_regex`), not
  `lazy_static` (deliberately removed in this fork in favor of `std::sync::OnceLock`).
- Errors use `thiserror`. Two error types: top-level `Error` (in `lib.rs`) and the serializer's
  `FormatError` (in `formatter.rs`).

## CI clippy gate
CI runs clippy with `--all-features` only (`cargo clippy --all-features --tests -- -D clippy::all`).
`--no-default-features` clippy has a known, pre-existing `needless_lifetimes` warning on
`impl<'a, W> Serializer for &'a mut Formatter<W>` (src/formatter.rs) — the `'a` is needed by the
json-feature associated types but unused when `json` is off. Not gated by CI; left untouched.

## Adding a new format
Implement the `Format` trait. The only required method is `iter_args`, returning an iterator of
`ArgumentSpec`. Build specs with the builder-style setters (position, width, precision, alignment,
etc.). See `NoopFormat` in `lib.rs` for the minimal example and `python.rs`/`curly.rs` for
regex-driven parsers.

## Python precision semantics (PR #5)
`%s`/`%r` set `ArgumentSpec::precision_truncates` (via `with_precision_truncates`); precision then
truncates the converted output to N chars post-format (type-independent). Numeric conversions forward
precision to `std::fmt` (float decimals). Width is measured in chars, not bytes. Known gaps: `%.5d`
zero-pad and `%.3g` significant-digits still defer to std::fmt (conversion type collapses to Display).

## Tests are the behavioral spec
Integration tests in `tests/` define expected behavior:
- `tests/test_python.rs` — Python `%` formatting incl. width/precision/truncation.
- `tests/test_curly.rs` — curly-brace formatting.
- `tests/test_error_messages.rs` — pins the EXACT `Display` text of `Error`/`FormatError`.
  Any change to parsing or error wording will require updating this file.
