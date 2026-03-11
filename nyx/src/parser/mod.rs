pub mod audio;
pub mod image;
pub mod text;

use crate::error::NyxResult;
use crate::domains::{AudioDomain, ImageDomain, TextDomain};
use std::path::Path;

/// Parse audio file (PCM WAV, MP3, etc.)
pub fn parse_audio<P: AsRef<Path>>(path: P) -> NyxResult<AudioDomain> {
    audio::parse_wav(path)
}

/// Parse image file (PNG, JPG, etc.)
pub fn parse_image<P: AsRef<Path>>(path: P) -> NyxResult<ImageDomain> {
    image::parse_image(path)
}

/// Parse text file
pub fn parse_text<P: AsRef<Path>>(path: P) -> NyxResult<TextDomain> {
    text::parse_text(path)
}
