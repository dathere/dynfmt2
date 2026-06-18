# Project Core

`dynfmt2` — a Rust crate for **runtime** string formatting (format string + args validated at
runtime, unlike compile-time `std::fmt`). Fork of the unmaintained `dynfmt` crate. Published to
crates.io as `dynfmt2`. Repo: github.com/dathere/dynfmt2.

## Source map

- `src/lib.rs` (~750 lines) — the heart. Defines the `Format` trait, `ArgumentSpec`,
  `FormatArgs`/`ArgumentAccess`, `Position`/`Count`/`Alignment`/`FormatType` enums, top-level
  `Error`, and `NoopFormat`.
- `src/formatter.rs` (~970 lines) — `Formatter<W>`, a `serde::Serializer` that renders argument
  values into the output buffer applying width/precision/alignment/sign. `FmtProxy`, `FormatError`.
- `src/python.rs` — `PythonFormat` parser (`printf`/`%`-style), `PYTHON_RE`.
- `src/curly.rs` — `SimpleCurlyFormat` parser (`{}`-style), `CURLY_RE`.
- `tests/` — integration tests are the behavioral spec (see `mem:conventions`).

## Core architecture

One trait drives everything: `Format::iter_args(format_str)` is the ONLY required method per
implementation; it returns an iterator of `ArgumentSpec`s (one per placeholder). The default
`Format::format()` walks specs, copies literal text between them, and calls `spec.format_into()`.
Adding a new format = implement `iter_args`; the rest is shared.

`ArgumentSpec` bridges parser→formatter: carries byte `range` in the format string, a `Position`
(positional index or named key), and `std::fmt`-style params (width, precision, alignment, sign,
fill, etc.). `width`/`precision` may be `Count::Value` or `Count::Ref` (taken from another arg).

Argument values are `erased_serde` trait objects serialized THROUGH `Formatter<W>` (the serde
`Serializer`), which is how arbitrary `Serialize` values get rendered.

Regexes are compiled once into a `OnceLock` (`get_python_regex`/`get_curly_regex`). This fork
deliberately dropped `lazy_static` for `std::sync::OnceLock`.

See also: `mem:tech_stack`, `mem:suggested_commands`, `mem:conventions`, `mem:task_completion`.
Project also has a `CLAUDE.md` at repo root with overlapping guidance.