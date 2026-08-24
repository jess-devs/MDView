//! Reads files from disk and decides whether they are readable text.
//! Owns no knowledge of GPUI or of what anything looks like.

use std::{fs, io, path::Path};

/// Why a document could not be shown (RF-17). `app` turns this into the
/// message the user sees; this module only classifies the failure.
pub enum LoadError {
    NotFound,
    PermissionDenied,
    NotText,
    /// Any other read failure (e.g. the path is a directory).
    Unreadable,
}

/// Reads a Markdown file from disk as UTF-8 text.
pub fn load(path: impl AsRef<Path>) -> Result<String, LoadError> {
    let bytes = fs::read(path.as_ref()).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => LoadError::NotFound,
        io::ErrorKind::PermissionDenied => LoadError::PermissionDenied,
        _ => LoadError::Unreadable,
    })?;
    String::from_utf8(bytes).map_err(|_| LoadError::NotText)
}
