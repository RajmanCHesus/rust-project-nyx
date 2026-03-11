use crate::domains::{ImageDomain, AudioDomain};
use crate::error::{NyxError, NyxResult};
use crate::transform::Transformer;

/// Transform image to audio (inverse spectrogram / pixel-to-frequency mapping)
pub struct ImageToAudioTransformer;

impl Transformer<ImageDomain, AudioDomain> for ImageToAudioTransformer {
    fn transform(&self, _input: ImageDomain) -> NyxResult<AudioDomain> {
        // TODO: Implement inverse spectrogram + iFFT
        Err(NyxError::TransformError(
            "ImageToAudio transformer not yet implemented".to_string(),
        ))
    }
}
