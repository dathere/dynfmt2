//! Implementation for simple format strings using curly braces.
//!
//! See [`SimpleCurlyFormat`] for more information.
//!
//! [`SimpleCurlyFormat`]: struct.SimpleCurlyFormat.html

use regex::{CaptureMatches, Captures, Regex};
use std::sync::OnceLock;

use crate::{ArgumentResult, ArgumentSpec, Error, Format, Position};

static CURLY_RE: OnceLock<Regex> = OnceLock::new();

fn get_curly_regex() -> &'static Regex {
    CURLY_RE.get_or_init(|| Regex::new(r"\{(?P<key>\w+)?\}").unwrap())
}

fn parse_position(key: &str) -> Position<'_> {
    key.parse().map_or_else(|_| Position::Key(key), Position::Index)
}

fn parse_next(captures: Captures<'_>) -> ArgumentSpec<'_> {
    let position = captures
        .name("key")
        .map_or_else(|| Position::Auto, |m| parse_position(m.as_str()));

    let group = captures.get(0).unwrap();
    ArgumentSpec::new(group.start(), group.end()).with_position(position)
}

/// Format argument iterator for [`SimpleCurlyFormat`].
///
/// [`SimpleCurlyFormat`]: struct.SimpleCurlyFormat.html
#[derive(Debug)]
pub struct SimpleCurlyIter<'f> {
    captures: CaptureMatches<'static, 'f>,
}

impl<'f> SimpleCurlyIter<'f> {
    fn new(format: &'f str) -> Self {
        SimpleCurlyIter {
            captures: get_curly_regex().captures_iter(format),
        }
    }
}

impl<'f> Iterator for SimpleCurlyIter<'f> {
    type Item = ArgumentResult<'f>;

    fn next(&mut self) -> Option<Self::Item> {
        self.captures.next().map(|capture| Ok(parse_next(capture)))
    }
}

/// Format implementation for simple curly brace based format strings.
///
/// This syntax is a subset of what Python 3, Rust, .NET and many logging libraries use. Each
/// argument is formated in display mode.
///
///   1. `{}`: Refers to the next positional argument.
///   2. `{0}`: Refers to the argument at index `0`.
///   3. `{name}`: Refers to the named argument with key `"name"`.
///
/// # Example
///
/// ```rust
/// use dynfmt2::{Format, SimpleCurlyFormat};
///
/// let formatted = SimpleCurlyFormat.format("hello, {}", &["world"]);
/// assert_eq!("hello, world", formatted.expect("formatting failed"));
/// ```
#[derive(Debug)]
pub struct SimpleCurlyFormat;

impl<'f> Format<'f> for SimpleCurlyFormat {
    type Iter = SimpleCurlyIter<'f>;

    fn iter_args(&self, format: &'f str) -> Result<Self::Iter, Error<'f>> {
        Ok(SimpleCurlyIter::new(format))
    }
}
