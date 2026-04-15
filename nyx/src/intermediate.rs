/// Phase 7: Intermediate Representation Layer
/// 
/// This layer defines the semantic domains that nyx works with.
/// Each type represents a distinct kind of data, independent of format.
/// 
/// Design principle: Transform operates on these types, not on file bytes.
/// 
/// Examples:
/// - WAV file → TimeSignal (parsed via hound)
/// - PNG file → SpatialRaster (parsed via image crate)
/// - TimeSignal + FrequencySpectrum (FFT output)
/// - FrequencySpectrum → SpatialRaster (visualization)

use crate::matrix::Matrix;
use num_complex::Complex;

// ============================================================================
// SYMBOLIC DATA: Text, metadata, or any discrete symbol sequence
// ============================================================================

/// Symbolic data: UTF-8 text or discrete symbols
#[derive(Clone, Debug)]
pub struct SymbolicData {
    pub content: String,
    pub encoding: TextEncoding,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextEncoding {
    UTF8,          // Standard UTF-8
    ASCII,         // 7-bit ASCII subset
    Latin1,        // ISO-8859-1
}

impl SymbolicData {
    pub fn new(content: String) -> Self {
        SymbolicData {
            content,
            encoding: TextEncoding::UTF8,
        }
    }

    pub fn with_encoding(content: String, encoding: TextEncoding) -> Self {
        SymbolicData { content, encoding }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

// ============================================================================
// TIME SIGNAL: Audio waveform samples (PCM or parametric)
// ============================================================================

/// Time-domain signal: audio samples with metadata
#[derive(Clone, Debug)]
pub struct TimeSignal {
    pub samples: Vec<f32>,      // PCM samples, amplitude ∈ [-1.0, 1.0]
    pub sample_rate: u32,       // Hz (44100, 48000, etc.)
    pub channels: u32,          // 1 (mono), 2 (stereo), etc.
    pub duration_secs: f32,     // Computed from len/sample_rate
}

impl TimeSignal {
    pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        let num_samples = samples.len() as f32;
        let duration_secs = num_samples / sample_rate as f32;
        TimeSignal {
            samples,
            sample_rate,
            channels: 1,
            duration_secs,
        }
    }

    pub fn with_channels(mut self, channels: u32) -> Self {
        self.channels = channels;
        self
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn bitrate(&self) -> u32 {
        self.sample_rate * self.channels * 16 // Assume 16-bit
    }

    /// Clamp samples to valid audio range [-1.0, 1.0]
    pub fn normalize(&mut self) {
        for sample in &mut self.samples {
            *sample = sample.clamp(-1.0, 1.0);
        }
    }
}

// ============================================================================
// SPATIAL RASTER: Image data (pixels in 2D grid)
// ============================================================================

/// RGB color pixel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    pub fn grayscale(value: u8) -> Self {
        Pixel {
            r: value,
            g: value,
            b: value,
        }
    }

    pub fn from_f32(value: f32) -> Self {
        let clamped = (value.clamp(0.0, 1.0) * 255.0) as u8;
        Self::grayscale(clamped)
    }
}

/// 2D image in spatial/pixel domain
#[derive(Clone, Debug)]
pub struct SpatialRaster {
    pub pixels: Matrix<Pixel>,  // width × height grid of RGB pixels
    pub width: u32,
    pub height: u32,
}

impl SpatialRaster {
    pub fn new(pixels: Matrix<Pixel>, width: u32, height: u32) -> Self {
        SpatialRaster {
            pixels,
            width,
            height,
        }
    }

    pub fn from_matrix(matrix: Matrix<Pixel>) -> Self {
        let (rows, cols) = matrix.dimensions();
        SpatialRaster {
            pixels: matrix,
            width: cols as u32,
            height: rows as u32,
        }
    }

    pub fn grayscale_from_matrix(matrix: Matrix<f32>) -> Self {
        let (rows, cols) = matrix.dimensions();
        let pixel_data: Vec<Pixel> = matrix
            .data()
            .iter()
            .map(|&val| Pixel::from_f32(val))
            .collect();

        let pixel_matrix = Matrix::new(pixel_data, rows, cols)
            .expect("SpatialRaster: pixel matrix dimensions mismatch");
        
        SpatialRaster {
            pixels: pixel_matrix,
            width: cols as u32,
            height: rows as u32,
        }
    }

    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// FREQUENCY SPECTRUM: Frequency-domain representation
// ============================================================================

/// Frequency-domain representation: complex numbers per frequency bin
#[derive(Clone, Debug)]
pub struct FrequencySpectrum {
    pub bins: Matrix<Complex<f32>>,  // frequency bins (magnitude + phase)
    pub sample_rate: u32,            // Original audio sample rate (Hz)
    pub fft_size: usize,             // Size of each FFT frame
    pub time_frames: usize,          // Number of frames (STFT)
}

impl FrequencySpectrum {
    pub fn new(
        bins: Matrix<Complex<f32>>,
        sample_rate: u32,
        fft_size: usize,
        time_frames: usize,
    ) -> Self {
        FrequencySpectrum {
            bins,
            sample_rate,
            fft_size,
            time_frames,
        }
    }

    pub fn freq_resolution(&self) -> f32 {
        self.sample_rate as f32 / self.fft_size as f32
    }

    pub fn nyquist_freq(&self) -> f32 {
        self.sample_rate as f32 / 2.0
    }

    /// Extract magnitude spectrum (ignore phase)
    pub fn magnitude_spectrum(&self) -> Matrix<f32> {
        let (rows, cols) = self.bins.dimensions();
        let magnitudes: Vec<f32> = self.bins.data().iter().map(|c| c.norm()).collect();
        Matrix::new(magnitudes, rows, cols)
            .expect("FrequencySpectrum: magnitude extraction failed")
    }

    /// Extract phase spectrum
    pub fn phase_spectrum(&self) -> Matrix<f32> {
        let (rows, cols) = self.bins.dimensions();
        let phases: Vec<f32> = self.bins.data().iter().map(|c| c.arg()).collect();
        Matrix::new(phases, rows, cols)
            .expect("FrequencySpectrum: phase extraction failed")
    }

    pub fn num_frequency_bins(&self) -> usize {
        let (rows, _) = self.bins.dimensions();
        rows
    }
}

// ============================================================================
// BINARY BLOB: Opaque binary data (for extensibility)
// ============================================================================

/// Opaque binary data: for formats not yet abstracted
#[derive(Clone, Debug)]
pub struct BinaryBlob {
    pub data: Vec<u8>,
    pub format_hint: String,  // e.g. "MP3", "FLAC", "Custom"
}

impl BinaryBlob {
    pub fn new(data: Vec<u8>, format_hint: String) -> Self {
        BinaryBlob { data, format_hint }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ============================================================================
// UNIFIED INTERMEDIATE DATA TYPE
// ============================================================================

/// The complete intermediate representation: any data type nyx works with
#[derive(Clone, Debug)]
pub enum IntermediateData {
    Symbolic(SymbolicData),
    TimeSig(TimeSignal),
    Spatial(SpatialRaster),
    Frequency(FrequencySpectrum),
    Binary(BinaryBlob),
}

impl IntermediateData {
    pub fn type_name(&self) -> &str {
        match self {
            IntermediateData::Symbolic(_) => "SymbolicData",
            IntermediateData::TimeSig(_) => "TimeSignal",
            IntermediateData::Spatial(_) => "SpatialRaster",
            IntermediateData::Frequency(_) => "FrequencySpectrum",
            IntermediateData::Binary(_) => "BinaryBlob",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_signal() {
        let samples = vec![0.0, 0.5, 1.0, 0.5];
        let signal = TimeSignal::new(samples, 44100);
        assert_eq!(signal.len(), 4);
        assert_eq!(signal.sample_rate, 44100);
        assert!(signal.duration_secs > 0.0);
    }

    #[test]
    fn test_pixel() {
        let p = Pixel::new(255, 128, 64);
        assert_eq!(p.r, 255);
        let g = Pixel::grayscale(128);
        assert_eq!(g.g, 128);
    }

    #[test]
    fn test_symbolic_data() {
        let text = SymbolicData::new("Hello, nyx!".to_string());
        assert_eq!(text.len(), 11);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_frequency_spectrum() {
        let bins = Matrix::filled(Complex::new(0.1, 0.0), 513, 10);
        let spec = FrequencySpectrum::new(bins, 44100, 1024, 10);
        assert_eq!(spec.sample_rate, 44100);
        assert!(spec.freq_resolution() > 0.0);
        assert!(spec.nyquist_freq() > 0.0);
    }
}
