//! Create a test spectrogram image that converts to audible audio
//! 
//! This generates a simple spectrogram PNG where pixel brightness represents
//! frequency energy, designed to produce a recognizable sound when converted to audio.

use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    // Create a test spectrogram: horizontal bands of energy 
    // representing different frequencies over time
    let width = 512;   // time frames
    let height = 512;  // frequency bins
    
    let mut img: RgbImage = ImageBuffer::new(width, height);
    
    // Add bright horizontal bands (pure tones at different frequencies)
    // Band 1: Low frequency (bottom 1/4 of image) - bright white
    for t in 0..width {
        for f in 0..height/4 {
            img.put_pixel(t as u32, f as u32, Rgb([255, 255, 255]));
        }
    }
    
    // Band 2: Mid frequency (next 1/4) - bright white for first half of time
    for t in 0..width/2 {
        for f in height/4..height/2 {
            img.put_pixel(t as u32, f as u32, Rgb([255, 255, 255]));
        }
    }
    
    // Band 3: High frequency (top 1/4) - bright white for last half of time
    for t in width/2..width {
        for f in 3*height/4..height {
            img.put_pixel(t as u32, f as u32, Rgb([255, 255, 255]));
        }
    }
    
    img.save("test_spectrogram.png")
        .expect("Failed to save test spectrogram");
    
    println!("✓ Created test_spectrogram.png");
    println!("  Size: {}×{} pixels", width, height);
    println!("  Contains bright bands representing tones at different frequencies");
    println!();
    println!("Convert to audio with:");
    println!("  cargo run --example png_to_wav -- test_spectrogram.png test_spectrogram.wav");
}
