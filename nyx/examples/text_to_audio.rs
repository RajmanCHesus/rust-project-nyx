//! Example: Text ! WAV Audio
//!
//! Demonstrates the forward transformation: taking text,
//! synthesizing audio via additive synthesis, and rendering as WAV.
//!
//! Usage:
//!    cargo run --example text_to_audio -- <input.txt> <output.wav>
//!

use std::env;
use nyx::error::NyxResult;
use nyx::parser::parse_text;
use nyx::domains::TextDomain;
use nyx::transform::text_to_audio::TextToAudioTransformer;
use nyx::transform::Transformer;
use nyx::render::{Renderer, audio::AudioRenderer};

fn main() -> NyxResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.txt> <output.wav>", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  input.txt   Path to input text file");
        eprintln!("  output.wav  Path to output WAV file");
        std::process::exit(1);
    } 

    let input_txt = &args[1];
    let output_wav = &args[2];

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║               Text → WAV Audio Synthesizer               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Step 1: Parse text file
    println!("Step 1: Loading text file...");
    println!("  Input: {}", input_txt);
    let text_domain = parse_text(input_txt)?;

    let TextDomain::Plain(text) = &text_domain;
    println!("  ✓ Loaded: {} characters", text.len());

    // Step 2: Compute audio (Additive synthesis)
    println!("Step 2: Synthesizing audio...");
    let transformer = TextToAudioTransformer::default();
    let audio_domain = transformer.transform(text_domain)?;

    if let nyx::domains::AudioDomain::PCM { samples, sample_rate } = &audio_domain {
        println!("  ✓ Synthesis complete: {} samples", samples.len());
        let duration = samples.len() as f32 / sample_rate as f32;
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Duration: {:.2} seconds\n", duration);
    }

    // Step 3: Render to WAV
    println!("Step 3: Rendering audio to WAV...");
    println!("  Output: {}", output_wav);
    println!("  Format: PCM WAV");

    let renderer = AudioRenderer::default();
    renderer.render(audio_domain, output_wav)?;
    println!("  ✓ Successfully saved to {}\n", output_wav);

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("✓ Conversion complete!");
    println!("╚══════════════════════════════════════════════════════════╝");

    Ok(())
}
