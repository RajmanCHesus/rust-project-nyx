//! Example: WAV Audio → PNG Spectrogram
//!
//! Demonstrates the forward transformation: taking audio (WAV), 
//! computing the spectrogram (STFT + FFT), and rendering as PNG.
//!
//! Usage:
//!    cargo run --example audio_to_image -- <input.wav> <output.png> [fft_size] [hop_size]
//!
//! Example:
//!    cargo run --example audio_to_image --release -- 1.wav 1.png 1024 512

use std::env;
use nyx::error::NyxResult;
use nyx::parser::audio::parse_wav;
use nyx::domains::AudioDomain;
use nyx::transform::audio_to_image::AudioToSpectrogramTransformer;
use nyx::transform::Transformer;
use nyx::render::{Renderer, image::SpectrogramRenderer};

fn main() -> NyxResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.wav> <output.png> [fft_size] [hop_size]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  input.wav   Path to input WAV file");
        eprintln!("  output.png  Path to output spectrogram PNG");
        eprintln!("  fft_size    FFT window size (default: 1024)");
        eprintln!("  hop_size    FFT hop size in samples (default: 512)");
        std::process::exit(1);
    }

    let input_wav = &args[1];
    let output_png = &args[2];
    let fft_size = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1024);
    let hop_size = args
        .get(4)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(512);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║        WAV Audio → PNG Spectrogram Visualization         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Step 1: Parse WAV file
    println!("Step 1: Loading audio file...");
    println!("  Input: {}", input_wav);
    let audio_domain = parse_wav(input_wav)?;

    if let AudioDomain::PCM { samples, sample_rate } = &audio_domain {
        println!("  ✓ Loaded: {} samples", samples.len());
        let duration = samples.len() as f32 / *sample_rate as f32;
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Duration: {:.2} seconds\n", duration);
    }

    // Step 2: Compute spectrogram (STFT)
    println!("Step 2: Computing spectrogram (STFT)...");
    println!("  FFT size: {} samples", fft_size);
    println!("  Hop size: {} samples", hop_size);
    
    let transformer = AudioToSpectrogramTransformer::new(fft_size, hop_size);
    let frequency_domain = transformer.transform(audio_domain)?;

    if let nyx::domains::ImageDomain::Frequency(freq_matrix) = &frequency_domain {
        let (freq_bins, time_frames) = freq_matrix.dimensions();
        println!("  ✓ Computed spectrogram: {} frequency bins × {} time frames", freq_bins, time_frames);
        println!("  Frequency resolution: {:.2} Hz/bin\n", 44100.0 / fft_size as f32);
    }

    // Step 3: Render to PNG
    println!("Step 3: Rendering spectrogram to PNG...");
    println!("  Output: {}", output_png);
    println!("  Format: RGB PNG (magnitude + phase encoding)");
    
    let renderer = SpectrogramRenderer::default();
    renderer.render(frequency_domain, output_png)?;
    println!("  ✓ Successfully saved to {}\n", output_png);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("✓ Conversion complete!");
    println!("╚══════════════════════════════════════════════════════════╝");

    Ok(())
}
