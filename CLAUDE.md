# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`dynfmt2` is a Rust crate for formatting strings dynamically, where the format string and
arguments are validated at **runtime** (not compile time, unlike `std::fmt`). It is a fork of the
unmaintained [`dynfmt`](https://github.com/jan-auer/dynfmt) crate. Published to crates.io as
`dynfmt2`.

## Commands

```bash
# Build
cargo build
cargo build --all-features

# Test (CI runs both of these — a change must pass both)
cargo test --all-features
cargo test --no-default-features        # the json feature is off here; python's %r will error

# Run a single test
cargo test --all-features test_width_formatting
cargo test --all-features --test test_python    # one integration test file

# Lint exactly as CI does (RUSTFLAGS=-Dwarnings; warnings fail the build)
cargo fmt --all -- --check
cargo clippy --all-features --tests -- -D clippy::all
```

CI (`.github/workflows/ci.yml`) gates on three jobs: fmt+clippy, `test --all-features`, and
`test --no-default-features`. Always verify a change against both the all-features and
no-default-features feature sets — a lot of code is behind `#[cfg(feature = ...)]`.

## Cargo Features

- `json` **(default)** — enables JSON serialization of complex argument values via `serde_json`.
  Python's `%r` (repr) conversion depends on this; without it, such values error.
- `python` — `printf`-style `%`-formatting (Python 2 style). Pulls in `regex`.
- `curly` — `{}`-brace formatting (.NET / Rust-like subset). Pulls in `regex`.

## Architecture

The crate is built around one core trait and a generic formatting pipeline. To understand it, read
these together: `src/lib.rs` (the trait + the value serializer driver) and `src/formatter.rs` (the
serde `Serializer` that turns argument values into formatted text).

### The `Format` trait (`src/lib.rs`)

`Format::iter_args(format_str)` is the **only required method** each format implementation provides.
It returns an iterator of `ArgumentSpec`s — one per placeholder found in the format string. The
default `Format::format(...)` method drives the whole pipeline: it walks the specs, copies the
literal text between placeholders, and calls `spec.format_into(...)` for each placeholder. New
formats are added by implementing `iter_args`; everything else is shared.

### `ArgumentSpec` (`src/lib.rs`)

The bridge between a parser and the formatter. Each spec carries:
- the byte `range` it occupies in the format string (so `format()` knows what text to replace),
- a `position` (positional index or named key) identifying which argument to pull,
- and `std::fmt`-style parameters: `format` type, `alternate`, `add_sign`, `pad_zero`,
  `fill_char`, `alignment`, `width`, `precision`.

A parser builds specs with the builder-style setters; `width`/`precision` can be `Count::Value` or
`Count::Ref` (taken from another argument).

### The formatter (`src/formatter.rs`)

`Formatter<W>` implements `serde::Serializer`. Argument values (which are `erased_serde`
trait objects) are serialized **through** this formatter, which applies the spec's width/precision/
alignment/sign rules as it writes. This is how arbitrary `Serialize` values get rendered into the
output buffer. `FmtProxy` adapts `std::fmt` formatting params; `FormatError` is the serializer's
error type.

### Argument access (`src/lib.rs`)

`FormatArgs` is implemented for slices, `Vec`, `VecDeque`, and `HashMap`/`BTreeMap` (for named
arguments). `ArgumentAccess` wraps the argument container and resolves a spec's `Position` to a
concrete value lazily.

### Format implementations

Each lives in its own module and is just a parser producing `ArgumentSpec`s via a regex:
- `src/python.rs` — `PythonFormat`, `PYTHON_RE`. Parses `%(name)s`, `%5.2f`, `%r`, etc.
- `src/curly.rs` — `SimpleCurlyFormat`, `CURLY_RE`. Parses `{}`, `{0}`, `{name}`.
- `NoopFormat` (in `lib.rs`) — passes text through unchanged; the minimal trait example.

Regexes are compiled once and cached in a `OnceLock` (`get_python_regex` / `get_curly_regex`) —
note this fork deliberately replaced `lazy_static` with `std::sync::OnceLock` (stable since 1.70).

## Tests

Integration tests live in `tests/` and are the primary spec for formatter behavior:
- `tests/test_python.rs` — Python `%` formatting, including width/precision.
- `tests/test_curly.rs` — curly-brace formatting.
- `tests/test_error_messages.rs` — exact runtime error message wording (asserts on `Display`).

When changing parsing or error behavior, expect to update `test_error_messages.rs`, which pins the
exact text of `Error`/`FormatError` messages.

## Conventions

- MSRV is **Rust 1.85.0** (`rust-version` in `Cargo.toml`), required by the 2024 edition.
- Edition 2024.
- Keep new functionality feature-gated and ensure it compiles/tests under `--no-default-features`.
