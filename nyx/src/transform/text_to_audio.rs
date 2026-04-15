use num_complex::Complex;
use crate::matrix::Matrix;

/// Audio domain: PCM samples or frequency representation
#[derive(Clone, Debug)]
pub enum AudioDomain {
    /// Raw PCM samples at a given sample rate
    PCM {
        samples: Vec<f32>,
        sample_rate: u32,
    },
    /// Frequency domain representation (complex)
    Frequency(Matrix<Complex<f32>>),
}

/// Image domain: pixel data or frequency representation
#[derive(Clone, Debug)]
pub enum ImageDomain {
    /// RGB pixel data
    RGB(Matrix<Rgb>),
    /// Frequency domain representation (complex)
    Frequency(Matrix<Complex<f32>>),
}

/// Simple RGB color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    /// Grayscale: all channels equal
    pub fn grayscale(value: u8) -> Self {
        Rgb { r: value, g: value, b: value }
    }
}

/// Text domain (placeholder for future)
#[derive(Clone, Debug)]
pub enum TextDomain {
    Plain(String),
}
