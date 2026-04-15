//! Example: PNG Spectrogram → WAV Audio Reconstruction
//!
//! Demonstrates the inverse transformation: taking a spectrogram image (PNG),
//! extracting the frequency data, and reconstructing audio via inverse FFT.
//!
//! Usage:
//!    cargo run --example png_to_wav -- <input.png> <output.wav> [sample_rate] [hop_size]
//!
//! Example:
//!    cargo run --example png_to_wav --release -- spectrogram.png reconstructed.wav 44100 512

use std::env;
use nyx::error::NyxResult;
use nyx::parser;
use nyx::domains::{ImageDomain, AudioDomain};
use nyx::transform::image_to_audio::SpectrogramToAudioTransformer;
use nyx::transform::Transformer;
use nyx::render::{Renderer, audio::AudioRenderer};

fn main() -> NyxResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.png> <output.wav> [sample_rate] [hop_size]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  input.png      Path to input spectrogram (PNG grayscale)");
        eprintln!("  output.wav     Path to output WAV file");
        eprintln!("  sample_rate    Audio sample rate in Hz (default: 44100)");
        eprintln!("  hop_size       FFT hop size in samples (default: 512)");
        std::process::exit(1);
    }

    let input_png = &args[1];
    let output_wav = &args[2];
    let sample_rate = args
        .get(3)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(44100);
    let hop_size = args
        .get(4)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(512);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       PNG Spectrogram → WAV Audio Reconstruction         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Step 1: Parse PNG as image
    println!("Step 1: Loading PNG spectrogram...");
    println!("  Input: {}", input_png);
    let image_domain = parser::parse_image(input_png)?;

    let (width, height) = match &image_domain {
        ImageDomain::RGB(raster) => {
            let (h, w) = raster.dimensions();
            (w, h)
        }
        _ => {
            eprintln!("Error: Expected RGB image domain");
            std::process::exit(1);
        }
    };

    println!("  ✓ Loaded: {}×{} pixels", width, height);
    println!("  (Frequency bins × Time frames)\n");

    // Step 2: Convert RGB pixels to frequency spectrum
    println!("Step 2: Converting pixels to frequency spectrum...");
    println!("  Mapping: pixel grayscale value → complex magnitude");
    let frequency_spectrum_domain = image_to_frequency_spectrum(image_domain)?;

    if let ImageDomain::Frequency(freq_matrix) = &frequency_spectrum_domain {
        let (freq_bins, time_frames) = freq_matrix.dimensions();
        println!("  ✓ Created frequency spectrum: {} bins × {} frames", freq_bins, time_frames);
        let freq_res = sample_rate as f32 / (freq_bins as f32 * 2.0);
        println!("  Frequency resolution: {:.2} Hz/bin", freq_res);
        println!("  Nyquist frequency: {:.0} Hz\n", sample_rate as f32 / 2.0);
    }

    // Step 3: Inverse FFT to reconstruct audio
    println!("Step 3: Applying inverse FFT (iFFT)...");
    println!("  Reconstructing time-domain samples");
    println!("  Sample rate: {} Hz", sample_rate);
    println!("  Hop size: {} samples", hop_size);

    let transformer = SpectrogramToAudioTransformer::new(sample_rate, hop_size);
    let audio_domain = transformer.transform(frequency_spectrum_domain)?;

    if let AudioDomain::PCM { samples, .. } = &audio_domain {
        println!("  ✓ Reconstructed: {} samples", samples.len());
        let duration = samples.len() as f32 / sample_rate as f32;
        println!("  Duration: {:.2} seconds\n", duration);
    }

    // Step 4: Render to WAV file
    println!("Step 4: Rendering reconstructed audio to WAV...");
    println!("  Output: {}", output_wav);
    println!("  Format: PCM 16-bit, mono, {} Hz", sample_rate);

    let renderer = AudioRenderer::new(sample_rate);
    renderer.render(audio_domain, output_wav)?;
    println!("  ✓ Successfully saved to {}\n", output_wav);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("✓ Reconstruction complete!");
    println!("╚══════════════════════════════════════════════════════════╝");

    Ok(())
}

/// Convert RGB image pixels back to frequency spectrum (inverse grayscale encoding)
///
/// This reconstructs the magnitude spectrum from pixel brightness:
/// - Pixel value 0-255 → magnitude 0.0-1.0
/// - Stored in complex format (magnitude only, phase = 0)
fn image_to_frequency_spectrum(
    image_domain: ImageDomain,
) -> NyxResult<ImageDomain> {
    use nyx::matrix::Matrix;

    let pixels_matrix = match image_domain {
        ImageDomain::RGB(raster) => raster,
        _ => {
            return Err(nyx::error::NyxError::TransformError(
                "Expected RGB image domain".to_string(),
            ));
        }
    };

    // Convert pixel grayscale values to magnitude
    // Assume grayscale where R==G==B
    let (height, width) = pixels_matrix.dimensions();
    let magnitude_data: Vec<f32> = pixels_matrix
        .data()
        .iter()
        .map(|pixel| {
            // Average RGB channels (should be same for grayscale)
            let avg = ((pixel.r as f32 + pixel.g as f32 + pixel.b as f32) / 3.0) / 255.0;
            avg
        })
        .collect();

    // Create magnitude matrix and convert to complex spectrum
    let magnitude_matrix = Matrix::new(magnitude_data, height, width)?;
    let complex_data: Vec<num_complex::Complex<f32>> = magnitude_matrix
        .data()
        .iter()
        .map(|&mag| num_complex::Complex::new(mag, 0.0))
        .collect();

    // Create frequency domain representation
    let spectrum_matrix = Matrix::new(
        complex_data,
        height,
        width,
    )?;

    Ok(ImageDomain::Frequency(spectrum_matrix))
}
