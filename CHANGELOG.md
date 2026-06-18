# Changelog

## Unreleased

- Migrate to the Rust 2024 edition (MSRV is now 1.85.0).
- Fix Python `%`-formatting precision and width handling:
  - Apply precision for floats (`%.2f`, `%.*f`) and strings (`%.3s`), which was previously parsed then silently dropped.
  - Measure width in characters rather than bytes, so multi-byte UTF-8 (e.g. `[%5s]` of `"café"`) pads correctly.
  - Require a literal dot before the precision in the regex, so malformed specs like `%5,2f` are no longer accepted.
  - Truncate `%s`/`%r` output to the precision length regardless of the argument's Rust type (matching Python's `%s`/repr behavior).

## 0.3.0

- Fork of the unmaintained [`dynfmt`](https://github.com/jan-auer/dynfmt) crate, published to crates.io as [`dynfmt2`](https://crates.io/crates/dynfmt2).
- Add width formatting support to `PythonFormat`.
- Migrate to the Rust 2021 edition and bump dependencies.
- Replace the `lazy_static` dependency with `std::sync::OnceLock` (MSRV raised to 1.70).
- Numerous clippy and rustfmt cleanups; add `Debug` implementations.

## 0.1.5

- Update error derivation approach to use [thiserror](https://crates.io/crates/thiserror).

## 0.1.4

- Fix a panic in the Python parser. When a pipe (`|`) occurred in the format string, the formatter would panic with `"unknown conversion flag"` due to an invalid Regex.

## 0.1.3

- Remove use of deprecated `mem::uninitialized`.
- Add messages to invocations of `unreachable!()`.

## 0.1.2

Implement `std::error::Error` for `dynfmt::Error` (courtesy of @cecton).

## 0.1.1

Fixes broken parsing of python mapping names (e.g. `%(name)s`)

## 0.1.0

Initial Release
