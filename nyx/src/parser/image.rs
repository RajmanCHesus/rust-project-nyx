use crate::error::{NyxError, NyxResult};
use crate::domains::{ImageDomain, Rgb};
use crate::matrix::Matrix;
use std::path::Path;
use num_complex::Complex;

/// Parse PNG/JPG and return ImageDomain::RGB
pub fn parse_image<P: AsRef<Path>>(path: P) -> NyxResult<ImageDomain> {
    let img = image::open(path)
        .map_err(|e| NyxError::ParseError(format!("Failed to open image: {}", e)))?;

    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let mut pixels = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let pixel = rgb_img.get_pixel(col, row);
            pixels.push(Rgb::new(pixel[0], pixel[1], pixel[2]));
        }
    }

    let matrix = Matrix::new(pixels, height as usize, width as usize)?;
    Ok(ImageDomain::RGB(matrix))
}

/// Convert RGB spectrogram image back to frequency domain (complex values)
/// Decodes magnitude from R channel and phase from G,B channels
pub fn rgb_to_frequency<P: AsRef<Path>>(path: P) -> NyxResult<ImageDomain> {
    let img = image::open(path)
        .map_err(|e| NyxError::ParseError(format!("Failed to open image: {}", e)))?;

    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let mut complex_values = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let pixel = rgb_img.get_pixel(col, row);
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Decode magnitude from R channel (0-255 → 0.0-1.0)
            let magnitude = r / 255.0;
            
            // Decode phase from G,B channels
            // G gives coarse phase, B gives fine phase
            let phase_coarse = (g / 255.0) * 2.0 * std::f32::consts::PI - std::f32::consts::PI;
            let phase_fine = (b / 255.0) * (1.0 / 256.0); // Add sub-byte precision
            let phase = phase_coarse + phase_fine;
            
            // Convert polar (magnitude, phase) to rectangular (real, imag)
            let complex = Complex::from_polar(magnitude, phase);
            complex_values.push(complex);
        }
    }

    let matrix = Matrix::new(complex_values, height as usize, width as usize)?;
    Ok(ImageDomain::Frequency(matrix))
}
