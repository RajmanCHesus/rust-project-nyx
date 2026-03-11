use crate::domains::{AudioDomain, ImageDomain};
use crate::error::{NyxError, NyxResult};
use crate::transform::Transformer;
use crate::matrix::Matrix;
use num_complex::Complex;
use rustfft::FftPlanner;

/// Transform audio (PCM) to image (spectrogram)
pub struct AudioToSpectrogramTransformer {
    pub fft_size: usize,
    pub hop_size: usize,
}

impl AudioToSpectrogramTransformer {
    pub fn new(fft_size: usize, hop_size: usize) -> Self {
        AudioToSpectrogramTransformer { fft_size, hop_size }
    }

    pub fn default() -> Self {
        AudioToSpectrogramTransformer {
            fft_size: 1024,
            hop_size: 512,
        }
    }

    /// Compute Hann window
    fn hann_window(&self, size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                let n = i as f32;
                let n_size = size as f32;
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * n / (n_size - 1.0)).cos())
            })
            .collect()
    }

    /// Compute magnitude spectrum in dB (log scale)
    fn magnitude_db(complex: &Complex<f32>) -> f32 {
        let magnitude = complex.norm();
        let db = 20.0 * magnitude.log10().max(-100.0); // floor at -100dB
        db
    }
}

impl Transformer<AudioDomain, ImageDomain> for AudioToSpectrogramTransformer {
    fn transform(&self, input: AudioDomain) -> NyxResult<ImageDomain> {
        match input {
            AudioDomain::PCM { samples, sample_rate: _ } => {
                // Compute spectrogram via STFT + FFT
                let window = self.hann_window(self.fft_size);
                let mut frames = Vec::new();

                for start in (0..samples.len()).step_by(self.hop_size) {
                    let _end = (start + self.fft_size).min(samples.len());
                    let mut frame = vec![Complex::new(0.0, 0.0); self.fft_size];

                    // Apply window
                    for (i, &window_val) in window.iter().enumerate() {
                        if start + i < samples.len() {
                            frame[i] = Complex::new(samples[start + i] * window_val, 0.0);
                        }
                    }

                    // FFT
                    let mut planner = FftPlanner::new();
                    let fft = planner.plan_fft_forward(self.fft_size);
                    fft.process(&mut frame);

                    // Store only positive frequencies (up to Nyquist)
                    let nyquist = self.fft_size / 2 + 1;
                    let spectrum: Vec<f32> = frame[..nyquist]
                        .iter()
                        .map(|c| Self::magnitude_db(c))
                        .collect();

                    frames.push(spectrum);
                }

                if frames.is_empty() {
                    return Err(NyxError::TransformError(
                        "No frames computed from audio".to_string(),
                    ));
                }

                // Construct spectrogram matrix: frequency bins (rows) x time frames (cols)
                let freq_bins = self.fft_size / 2 + 1;
                let time_frames = frames.len();
                let mut spectrogram_data = vec![0.0; freq_bins * time_frames];

                for (t, frame) in frames.iter().enumerate() {
                    for (f, &mag) in frame.iter().enumerate() {
                        spectrogram_data[f * time_frames + t] = mag;
                    }
                }

                let matrix = Matrix::new(spectrogram_data, freq_bins, time_frames)?;

                // Convert to frequency domain representation
                let freq_data: Vec<Complex<f32>> = matrix
                    .data()
                    .iter()
                    .map(|&val| Complex::new(val, 0.0))
                    .collect();
                let freq_matrix: Matrix<Complex<f32>> = Matrix::new(
                    freq_data,
                    freq_bins,
                    time_frames,
                )?;

                Ok(ImageDomain::Frequency(freq_matrix))
            }
            _ => Err(NyxError::TransformError(
                "Audio must be in PCM domain for spectrogram".to_string(),
            )),
        }
    }
}
