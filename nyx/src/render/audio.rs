use crate::domains::AudioDomain;
use crate::error::{NyxError, NyxResult};
use crate::render::Renderer;

/// Render audio to WAV file
pub struct AudioRenderer {
    pub sample_rate: u32,
}

impl AudioRenderer {
    pub fn new(sample_rate: u32) -> Self {
        AudioRenderer { sample_rate }
    }

    pub fn default() -> Self {
        AudioRenderer {
            sample_rate: 44100,
        }
    }
}

impl Renderer<AudioDomain> for AudioRenderer {
    fn render(&self, input: AudioDomain, output_path: &str) -> NyxResult<()> {
        match input {
            AudioDomain::PCM { samples, .. } => {
                // Convert f32 samples to i16
                let i16_samples: Vec<i16> = samples
                    .iter()
                    .map(|&s| {
                        let clamped = s.clamp(-1.0, 1.0);
                        (clamped * i16::MAX as f32) as i16
                    })
                    .collect();

                // Write WAV file
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: self.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut writer = hound::WavWriter::create(output_path, spec)
                    .map_err(|e| NyxError::RenderError(format!("Failed to create WAV file: {}", e)))?;

                for sample in i16_samples {
                    writer
                        .write_sample(sample)
                        .map_err(|e| NyxError::RenderError(format!("Failed to write sample: {}", e)))?;
                }

                writer.finalize()
                    .map_err(|e| NyxError::RenderError(format!("Failed to finalize WAV file: {}", e)))?;

                Ok(())
            }
            _ => Err(NyxError::RenderError(
                "Audio must be in PCM domain for WAV rendering".to_string(),
            )),
        }
    }
}
