use crate::error::{NyxError, NyxResult};
use crate::domains::{ImageDomain, Rgb};
use crate::matrix::Matrix;
use std::path::Path;

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
