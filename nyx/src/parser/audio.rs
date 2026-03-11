use crate::error::{NyxError, NyxResult};
use crate::domains::AudioDomain;
use std::path::Path;

/// Parse WAV file and return AudioDomain::PCM
pub fn parse_wav<P: AsRef<Path>>(path: P) -> NyxResult<AudioDomain> {
    let reader = hound::WavReader::open(path)
        .map_err(|e| NyxError::ParseError(format!("Failed to open WAV file: {}", e)))?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;

    // Read samples (assumes 32-bit float or will convert)
    let samples: Vec<f32> = match spec.bits_per_sample {
        32 => {
            reader
                .into_samples::<i32>()
                .map(|s| {
                    s.map(|sample| sample as f32 / i32::MAX as f32)
                        .unwrap_or(0.0)
                })
                .collect()
        }
        16 => {
            reader
                .into_samples::<i16>()
                .map(|s| {
                    s.map(|sample| sample as f32 / i16::MAX as f32)
                        .unwrap_or(0.0)
                })
                .collect()
        }
        _ => {
            return Err(NyxError::ParseError(format!(
                "Unsupported bits per sample: {}",
                spec.bits_per_sample
            )))
        }
    };

    Ok(AudioDomain::PCM {
        samples,
        sample_rate,
    })
}
