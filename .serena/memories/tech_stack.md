# Tech Stack

- Language: Rust, edition 2024.
- No `rust-version`/MSRV is declared in `Cargo.toml`. The Rust floor is whatever edition 2024 and
  the language features in use require (e.g. `let`-chains, stabilized in 1.88).
- Build/package: Cargo.
- License: MIT.

## Dependencies (`Cargo.toml`)

- `erased-serde` 0.4 — type-erased serialization of argument values.
- `serde` 1 — values must be `Serialize`.
- `thiserror` 2.0 — error derivation.
- `serde_json` 1 (optional) — behind `json` feature.
- `regex` 1 (optional) — behind `python` and `curly` features.

## Cargo features

- `json` **(default)** — JSON serialization of complex argument values. Python's `%r` (repr)
  depends on this; without it such values error.
- `python` — `printf`/`%`-style formatting (Python 2 style). Pulls in `regex`.
- `curly` — `{}`-brace formatting (.NET/Rust-like subset). Pulls in `regex`.

Lots of code is `#[cfg(feature = ...)]`-gated; always validate against both `--all-features` and
`--no-default-features`.