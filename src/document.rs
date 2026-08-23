//! Reads files from disk and decides whether they are readable text.
//! Owns no knowledge of GPUI or of what anything looks like.

use std::{fs, io, path::Path};

/// Reads a Markdown file from disk as UTF-8 text.
pub fn load(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}
