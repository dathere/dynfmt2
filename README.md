# dynfmt2 - Dynamic Formatting in Rust

A crate for formatting strings dynamically.

> `dynfmt2` is a fork of the [`dynfmt`](https://github.com/jan-auer/dynfmt) crate, which has not been updated since Mar 2021, with some improvements and changes:
>
> - the `lazy_static` dependency is removed, and the code is updated to work using `std::sync::OnceLock` instead
> - MSRV is set to Rust 1.70.0 to use `std::sync::OnceLock`
> - Rust 2021 edition
> - bumped dependencies
> - numerous clippy lints applied

---

`dynfmt2` provides several implementations for formats that implement a subset of the
[`std::fmt`] facilities. Parsing of the format string and arguments checks are performed at
runtime. There is also the option to implement new formats.

The public API is exposed via the [`Format`] trait, which contains formatting helper functions
and lower-level utilities to interface with format strings. See the Features section for a list
of provided implementations.

## Usage

```rust
use dynfmt2::{Format, NoopFormat};

let formatted = NoopFormat.format("hello, world", &["unused"]);
assert_eq!("hello, world", formatted.expect("formatting failed"));
```

See the [`Format`] trait for more methods.

## Features

This crate ships with a set of features that either activate formatting capabilities or new
format implementations:

 - `json` **(default)**: Implements the serialization of complex structures via JSON. Certain
   formats, such as Python, also have a _representation_ format (`%r`) that makes use of this
   feature, if enabled. Without this feature, such values will cause an error.
 - `python`: Implements the `printf`-like format that python 2 used for formatting strings. See
   [`PythonFormat`] for more information.
 - `curly`: A simple format string syntax using curly braces for arguments. Similar to .NET and
   Rust, but much less capable. See [`SimpleCurlyFormat`] for mor information.

## Extensibility

Implement the [`Format`] trait to create a new format. The only required method is `iter_args`,
which must return an iterator over [`ArgumentSpec`] structs. Based on the capabilities of the
format, the specs can be parameterized with formatting parameters.

```rust
use std::str::MatchIndices;
use dynfmt2::{ArgumentSpec, Format, Error};

struct HashFormat;

impl<'f> Format<'f> for HashFormat {
    type Iter = HashIter<'f>;

    fn iter_args(&self, format: &'f str) -> Result<Self::Iter, Error<'f>> {
        Ok(HashIter(format.match_indices('#')))
    }
}

struct HashIter<'f>(MatchIndices<'f, char>);

impl<'f> Iterator for HashIter<'f> {
    type Item = Result<ArgumentSpec<'f>, Error<'f>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(index, _)| Ok(ArgumentSpec::new(index, index + 1)))
    }
}

let formatted = HashFormat.format("hello, #", &["world"]);
assert_eq!("hello, world", formatted.expect("formatting failed"));
```

[`std::fmt`]: https://doc.rust-lang.org/stable/std/fmt/
[`serde::Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`Format`]: https://docs.rs/dynfmt2/latest/dynfmt2/trait.Format.html
[`ArgumentSpec`]: https://docs.rs/dynfmt2/latest/dynfmt2/struct.ArgumentSpec.html
[`PythonFormat`]: https://docs.rs/dynfmt2/latest/dynfmt2/python/struct.PythonFormat.html
[`SimpleCurlyFormat`]: https://docs.rs/dynfmt2/latest/dynfmt2/curly/struct.SimpleCurlyFormat.html

License: MIT
