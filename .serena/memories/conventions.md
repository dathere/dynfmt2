# Conventions

- MSRV 1.70.0 — no std APIs newer than that. Edition 2021.
- Keep new functionality feature-gated (`#[cfg(feature = ...)]`) and ensure it still
  compiles/tests under `--no-default-features`.
- Cached regexes go through `OnceLock` helpers (`get_python_regex`/`get_curly_regex`), not
  `lazy_static` (deliberately removed in this fork).
- Errors use `thiserror`. Two error types: top-level `Error` (in `lib.rs`) and the serializer's
  `FormatError` (in `formatter.rs`).

## Adding a new format
Implement the `Format` trait. The only required method is `iter_args`, returning an iterator of
`ArgumentSpec`. Build specs with the builder-style setters (position, width, precision, alignment,
etc.). See `NoopFormat` in `lib.rs` for the minimal example and `python.rs`/`curly.rs` for
regex-driven parsers.

## Tests are the behavioral spec
Integration tests in `tests/` define expected behavior:
- `tests/test_python.rs` — Python `%` formatting incl. width/precision.
- `tests/test_curly.rs` — curly-brace formatting.
- `tests/test_error_messages.rs` — pins the EXACT `Display` text of `Error`/`FormatError`.
  Any change to parsing or error wording will require updating this file.