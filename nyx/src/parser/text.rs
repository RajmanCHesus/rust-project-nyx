use crate::error::{NyxError, NyxResult};
use crate::domains::TextDomain;
use std::path::Path;
use std::fs;

/// Parse text file
pub fn parse_text<P: AsRef<Path>>(path: P) -> NyxResult<TextDomain> {
    let content = fs::read_to_string(path)
        .map_err(|e| NyxError::ParseError(format!("Failed to read text file: {}", e)))?;

    Ok(TextDomain::Plain(content))
}
