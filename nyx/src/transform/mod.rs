pub mod audio_to_image;
pub mod image_to_audio;

use crate::error::NyxResult;

/// Generic transformer trait
pub trait Transformer<I, O> {
    /// Transform input to output
    fn transform(&self, input: I) -> NyxResult<O>;
}
