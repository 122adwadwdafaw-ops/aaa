//! JSXBIN input decoding. See README.md for format coverage and provenance.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod decode;
mod reader;

use std::fmt;

/// Resource limits apply before JavaScript is handed to the host runtime.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub input_bytes: usize,
    pub output_bytes: usize,
    pub nodes: usize,
    pub depth: usize,
    pub string_units: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self { input_bytes: 16 * 1024 * 1024, output_bytes: 32 * 1024 * 1024, nodes: 500_000, depth: 96, string_units: 16 * 1024 * 1024 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// Byte offset in the original encoded input (not a reconstructed source line).
    pub offset: usize,
    pub message: String,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSXBIN at byte {}: {}", self.offset, self.message)
    }
}
impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Decoded {
    pub source: String,
    pub version: &'static str,
}

pub fn is_jsxbin(source: &str) -> bool {
    source.trim_start_matches('\u{feff}').trim_start().starts_with("@JSXBIN")
}

pub fn decode(source: &str) -> Result<Decoded> {
    decode_with_limits(source, Limits::default())
}

pub fn decode_with_limits(source: &str, limits: Limits) -> Result<Decoded> {
    // Callers may tighten budgets, but must not disable the stack-safety bound.
    if limits.depth > 96 {
        return Err(Error { offset: 0, message: "decoder depth limit cannot exceed 96".into() });
    }
    decode::decode(source, limits)
}
