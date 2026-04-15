use crate::domains::{ImageDomain, AudioDomain};
use crate::error::{NyxError, NyxResult};
use crate::transform::Transformer;
use num_complex::Complex;
use rustfft::FftPlanner;

/// Transform spectrogram image back to audio (inverse FFT)
pub struct SpectrogramToAudioTransformer {
    pub sample_rate: u32,
    pub hop_size: usize,
}

impl SpectrogramToAudioTransformer {
    pub fn new(sample_rate: u32, hop_size: usize) -> Self {
        SpectrogramToAudioTransformer { sample_rate, hop_size }
    }

    pub fn default() -> Self {
        SpectrogramToAudioTransformer {
            sample_rate: 44100,
            hop_size: 512,
        }
    }

    /// Compute Hann window (same as forward transform)
    fn hann_window(&self, size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                let n = i as f32;
                let n_size = size as f32;
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * n / (n_size - 1.0)).cos())
            })
            .collect()
    }
    
    /// Use a simple low-pass filter to smooth high-frequency noise from reconstruction
    fn smooth_samples(samples: &mut Vec<f32>) {
        if samples.len() < 3 {
            return;
        }
        
        // Simple 3-tap low-pass filter to reduce fizz
        // Kernel: [0.25, 0.5, 0.25] normalized
        let mut smoothed = samples.clone();
        for i in 1..samples.len() - 1 {
            smoothed[i] = 0.25 * samples[i - 1] + 0.5 * samples[i] + 0.25 * samples[i + 1];
        }
        *samples = smoothed;
    }
}

impl Transformer<ImageDomain, AudioDomain> for SpectrogramToAudioTransformer {
    fn transform(&self, input: ImageDomain) -> NyxResult<AudioDomain> {
        match input {
            ImageDomain::Frequency(freq_matrix) => {
                let (freq_bins, time_frames) = freq_matrix.dimensions();
                
                // Infer FFT size from frequency bins
                // For real FFT: freq_bins = fft_size / 2 + 1
                let fft_size = (freq_bins - 1) * 2;
                
                // Extract magnitude values from complex matrix
                let mut frames = Vec::new();
                for t in 0..time_frames {
                    let mut frame = vec![Complex::new(0.0, 0.0); fft_size];
                    
                    // Copy positive frequencies - USE ACTUAL PHASE FROM PNG
                    for f in 0..freq_bins {
                        if let Ok(cell) = freq_matrix.get(f, t) {
                            // Keep the magnitude AND phase as encoded in the PNG
                            frame[f] = *cell;
                        }
                    }
                    
                    // Mirror for negative frequencies (Hermitian symmetry)
                    // freq[N-k] = conj(freq[k]) for real signals
                    for k in 1..freq_bins.saturating_sub(1) {
                        let mirror_idx = fft_size - k;
                        if mirror_idx < fft_size {
                            frame[mirror_idx] = frame[k].conj();
                        }
                    }
                    
                    frames.push(frame);
                }
                
                // Inverse FFT each frame
                let mut ifft_frames = Vec::new();
                for mut frame in frames {
                    let mut planner = FftPlanner::new();
                    let ifft = planner.plan_fft_inverse(fft_size);
                    ifft.process(&mut frame);
                    
                    // Normalize by FFT size
                    for sample in &mut frame {
                        *sample /= fft_size as f32;
                    }
                    
                    ifft_frames.push(frame);
                }
                
                // Apply Hann window and overlap-add
                let window = self.hann_window(fft_size);
                let output_len = time_frames * self.hop_size + fft_size;
                let mut samples = vec![0.0f32; output_len];
                let mut window_sum = vec![0.0f32; output_len]; // for normalization
                
                for (t, frame) in ifft_frames.iter().enumerate() {
                    let start = t * self.hop_size;
                    
                    for (i, &sample) in frame.iter().enumerate() {
                        if start + i < samples.len() {
                            let windowed = sample * window[i];
                            samples[start + i] += windowed.re;
                            window_sum[start + i] += window[i] * window[i];
                        }
                    }
                }
                
                // Normalize by window sum to compensate for overlap-add
                for i in 0..samples.len() {
                    if window_sum[i] > 1e-6 {
                        samples[i] /= window_sum[i];
                    }
                }
                
                // Apply smoothing filter to reduce artifacts/fizz
                Self::smooth_samples(&mut samples);
                
                // Find maximum absolute value for normalization
                let max_abs = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                
                // Normalize to use full dynamic range (boost quiet signals)
                if max_abs > 1e-6 {
                    // Normalize to -1.0..1.0 range
                    for sample in &mut samples {
                        *sample /= max_abs;
                    }
                } else {
                    // If signal is essentially silent, apply some gain anyway
                    for sample in &mut samples {
                        *sample *= 10.0; // Apply 10x gain boost for very quiet signals
                    }
                }
                
                // Final amplification gain (moderate for better quality)
                // Using actual phase data, so don't need extreme boost
                // 1.5 = ~3.5dB for moderate amplification
                let gain = 16.5f32;
                for sample in &mut samples {
                    *sample *= gain;
                }
                
                // Clamp to valid audio range
                for sample in &mut samples {
                    *sample = sample.clamp(-1.0, 1.0);
                }
                
                Ok(AudioDomain::PCM {
                    samples,
                    sample_rate: self.sample_rate,
                })
            }
            _ => Err(NyxError::TransformError(
                "Image must be in frequency domain for audio synthesis".to_string(),
            )),
        }
    }
}
