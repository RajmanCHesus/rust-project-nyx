use std::fmt;

#[derive(Debug)]
pub enum NyxError {
    IoError(String),
    ParseError(String),
    TransformError(String),
    RenderError(String),
    InvalidInput(String),
}

impl fmt::Display for NyxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NyxError::IoError(msg) => write!(f, "IO Error: {}", msg),
            NyxError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            NyxError::TransformError(msg) => write!(f, "Transform Error: {}", msg),
            NyxError::RenderError(msg) => write!(f, "Render Error: {}", msg),
            NyxError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl std::error::Error for NyxError {}

pub type NyxResult<T> = Result<T, NyxError>;
