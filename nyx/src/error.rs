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

// Convert from external error types
impl From<hound::Error> for NyxError {
    fn from(err: hound::Error) -> Self {
        NyxError::IoError(format!("Hound error: {}", err))
    }
}

impl From<image::ImageError> for NyxError {
    fn from(err: image::ImageError) -> Self {
        NyxError::IoError(format!("Image error: {}", err))
    }
}

impl From<std::io::Error> for NyxError {
    fn from(err: std::io::Error) -> Self {
        NyxError::IoError(format!("IO error: {}", err))
    }
}
