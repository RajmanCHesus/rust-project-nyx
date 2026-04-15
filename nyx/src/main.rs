use clap::Parser;
use nyx::error::NyxResult;
use nyx::parser;
use nyx::transform::{Transformer, audio_to_image::AudioToSpectrogramTransformer, image_to_audio::SpectrogramToAudioTransformer, text_to_audio::TextToAudioTransformer};
use nyx::render::Renderer;
use nyx::render::image::SpectrogramRenderer;
use nyx::render::audio::AudioRenderer;

#[derive(Parser, Debug)]
#[command(name = "nyx")]
#[command(about = "Cross-modal transformation system", long_about = None)]
struct Args {
    /// Input file path
    #[arg(value_name = "FILE")]
    input: String,

    /// Output file path
    #[arg(value_name = "FILE")]
    output: String,

    /// Transformation mode: audio-to-image, image-to-audio, text-to-audio
    #[arg(short, long, default_value = "audio-to-image")]
    mode: String,

    /// FFT size (default: 1024)
    #[arg(long, default_value = "1024")]
    fft_size: usize,

    /// Hop size / stride (default: 512)
    #[arg(long, default_value = "512")]
    hop_size: usize,
}

fn main() -> NyxResult<()> {
    let args = Args::parse();
    
    match args.mode.as_str() {
        "audio-to-image" => {
            println!("Transforming audio → spectrogram: {} → {}", args.input, args.output);
            
            // Parse WAV file
            println!("  [1/3] Parsing audio...");
            let audio = parser::parse_audio(&args.input)?;
            
            // Transform to spectrogram
            println!("  [2/3] Computing spectrogram (FFT size: {}, hop: {})...", args.fft_size, args.hop_size);
            let transformer = AudioToSpectrogramTransformer::new(args.fft_size, args.hop_size);
            let spectrogram = transformer.transform(audio)?;
            
            // Render to PNG
            println!("  [3/3] Rendering spectrogram to PNG...");
            let renderer = SpectrogramRenderer::default();
            renderer.render(spectrogram, &args.output)?;
            
            println!("✓ Spectrogram saved to {}", args.output);
        }
        "image-to-audio" => {
            println!("Transforming spectrogram → audio: {} → {}", args.input, args.output);
            
            // Parse PNG spectrogram and decode to frequency domain
            println!("  [1/3] Parsing spectrogram image (decoding phase information)...");
            let image = parser::parse_spectrogram(&args.input)?;
            
            // Transform to audio via inverse spectrogram
            println!("  [2/3] Computing inverse spectrogram (iFFT, hop: {})...", args.hop_size);
            let transformer = SpectrogramToAudioTransformer::new(44100, args.hop_size);
            let audio = transformer.transform(image)?;
            
            // Render to WAV
            println!("  [3/3] Rendering audio to WAV...");
            let renderer = AudioRenderer::default();
            renderer.render(audio, &args.output)?;
            
            println!("✓ Audio saved to {}", args.output);
        }
        "text-to-audio" => {
            println!("Transforming text → audio: {} → {}", args.input, args.output);

            // Parse text
            println!("  [1/3] Parsing text...");
            let text_domain = parser::parse_text(&args.input)?;

            // Transform to audio
            println!("  [2/3] Synthesizing audio...");
            let transformer = TextToAudioTransformer::default();
            let audio = transformer.transform(text_domain)?;

            // Render to WAV
            println!("  [3/3] Rendering audio to WAV...");
            let renderer = AudioRenderer::default();
            renderer.render(audio, &args.output)?;

            println!("✓ Audio saved to {}", args.output);
        }
        _ => {
            eprintln!("Unknown mode: {}. Use 'audio-to-image', 'image-to-audio', or 'text-to-audio'", args.mode);
        }
    }

    Ok(())
}
