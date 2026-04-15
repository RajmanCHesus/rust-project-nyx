pub mod audio_to_image;
pub mod image_to_audio;

use crate::error::NyxResult;
use crate::intermediate::IntermediateData;

/// Generic transformer trait: works with intermediate data types
/// 
/// This trait defines the semantic contract for all transformations.
/// A transformer takes one intermediate type and produces another.
/// 
/// Example implementations:
/// - TimeSignal → FrequencySpectrum (FFT)
/// - FrequencySpectrum → TimeSignal (inverse FFT)
/// - FrequencySpectrum → SpatialRaster (spectrogram visualization)
/// - SpatialRaster → TimeSignal (pixel-to-audio synthesis)
pub trait Transform {
    /// Human-readable name of this transformer
    fn name(&self) -> &str;

    /// Transform input intermediate data to output
    fn transform(&self, input: IntermediateData) -> NyxResult<IntermediateData>;

    /// Optional: describe what input types this transformer accepts
    fn accepts(&self) -> &str {
        "Any"
    }

    /// Optional: describe what output types this transformer produces
    fn produces(&self) -> &str {
        "Any"
    }
}

// ============================================================================
// Legacy trait: kept for backward compatibility during refactoring
// ============================================================================

/// Generic transformer trait (legacy, being refactored to Transform)
/// 
/// This will be phased out as we migrate to IntermediateData model.
pub trait Transformer<I, O> {
    /// Transform input to output
    fn transform(&self, input: I) -> NyxResult<O>;
}
