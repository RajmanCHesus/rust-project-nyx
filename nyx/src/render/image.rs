use crate::domains::ImageDomain;
use crate::error::{NyxError, NyxResult};
use crate::render::Renderer;

/// Render spectrogram (frequency matrix) to PNG image
pub struct SpectrogramRenderer {
    pub normalize: bool,
}

impl SpectrogramRenderer {
    pub fn new(normalize: bool) -> Self {
        SpectrogramRenderer { normalize }
    }

    pub fn default() -> Self {
        SpectrogramRenderer { normalize: true }
    }

    /// Convert dB magnitude to grayscale pixel value (0-255)
    fn magnitude_to_pixel(&self, magnitude_db: f32, min_db: f32, max_db: f32) -> u8 {
        let normalized = (magnitude_db - min_db) / (max_db - min_db);
        let clamped = normalized.clamp(0.0, 1.0);
        (clamped * 255.0) as u8
    }
}

impl Renderer<ImageDomain> for SpectrogramRenderer {
    fn render(&self, input: ImageDomain, output_path: &str) -> NyxResult<()> {
        match input {
            ImageDomain::Frequency(matrix) => {
                // Extract magnitudes from complex matrix
                let magnitudes: Vec<f32> = matrix
                    .data()
                    .iter()
                    .map(|c| c.norm())
                    .collect();

                let (freq_bins, time_frames) = matrix.dimensions();

                // Compute min/max for normalization
                let min_mag = magnitudes
                    .iter()
                    .cloned()
                    .fold(f32::INFINITY, f32::min);
                let max_mag = magnitudes
                    .iter()
                    .cloned()
                    .fold(f32::NEG_INFINITY, f32::max);

                // Convert to image buffer
                let mut img_data = vec![0u8; freq_bins * time_frames * 3]; // RGB

                for (i, &mag) in magnitudes.iter().enumerate() {
                    let pixel_val = if self.normalize && max_mag > min_mag {
                        self.magnitude_to_pixel(mag, min_mag, max_mag)
                    } else {
                        (mag.clamp(0.0, 255.0)) as u8
                    };

                    let base_idx = i * 3;
                    img_data[base_idx] = pixel_val;
                    img_data[base_idx + 1] = pixel_val;
                    img_data[base_idx + 2] = pixel_val;
                }

                // Create image (frequency × time)
                let img = image::RgbImage::from_raw(time_frames as u32, freq_bins as u32, img_data)
                    .ok_or_else(|| NyxError::RenderError("Failed to create image buffer".to_string()))?;

                img.save(output_path)
                    .map_err(|e| NyxError::RenderError(format!("Failed to save PNG: {}", e)))?;

                Ok(())
            }
            _ => Err(NyxError::RenderError(
                "Image must be in frequency domain for spectrogram rendering".to_string(),
            )),
        }
    }
}
