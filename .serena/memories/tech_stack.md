# Tech Stack

- Language: Rust, edition 2021.
- MSRV: **Rust 1.70.0** (`rust-version` in `Cargo.toml`). Do not use newer std APIs. The 1.70 pin
  exists because the fork uses `std::sync::OnceLock` (replacing `lazy_static`).
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