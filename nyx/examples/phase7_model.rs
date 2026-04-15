/// Phase 7: Intermediate Model Example
/// 
/// This example demonstrates the new IntermediateData model and Transform trait.
/// It shows how data flows through semantic domains rather than file formats.

#[allow(dead_code)]
fn main() {
    // Example 1: Direct intermediate model usage
    example_intermediate_model();
    
    // Example 2: Transform pipeline (when fully implemented)
    example_transform_pipeline();
}

#[allow(dead_code)]
fn example_intermediate_model() {
    use nyx::intermediate::{TimeSignal, FrequencySpectrum, SpatialRaster, Pixel, SymbolicData};
    use nyx::matrix::Matrix;
    use num_complex::Complex;

    println!("=== Phase 7: Intermediate Data Model ===\n");

    // Create a simple time signal (440Hz sine, 1 second, 44.1kHz)
    let samples = (0..44100)
        .map(|i| {
            let t = i as f32 / 44100.0;
            (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.8
        })
        .collect::<Vec<f32>>();

    let time_signal = TimeSignal::new(samples, 44100);
    println!(
        "TimeSignal: {} samples @ {} Hz, {} seconds",
        time_signal.len(),
        time_signal.sample_rate,
        time_signal.duration_secs
    );
    println!("  Bitrate: {} bps\n", time_signal.bitrate());

    // Create a frequency spectrum (simulated FFT output)
    let spectrum_data = vec![Complex::new(0.1, 0.0); 513 * 10]; // 513 freq bins, 10 time frames
    let spectrum_matrix = Matrix::new(spectrum_data, 513, 10).unwrap();
    let frequency_spectrum = FrequencySpectrum::new(spectrum_matrix, 44100, 1024, 10);
    println!(
        "FrequencySpectrum: {} bins × {} frames",
        frequency_spectrum.num_frequency_bins(),
        frequency_spectrum.time_frames
    );
    println!(
        "  Frequency resolution: {:.1} Hz",
        frequency_spectrum.freq_resolution()
    );
    println!(
        "  Nyquist frequency: {:.0} Hz\n",
        frequency_spectrum.nyquist_freq()
    );

    // Create a spatial raster (image)
    let pixels = (0..512 * 256)
        .map(|i| {
            let val = ((i as f32 / (512.0 * 256.0)) * 255.0) as u8;
            Pixel::grayscale(val)
        })
        .collect::<Vec<_>>();
    let pixel_matrix = Matrix::new(pixels, 256, 512).unwrap();
    let spatial_raster = SpatialRaster::from_matrix(pixel_matrix);
    println!(
        "SpatialRaster: {}×{} pixels ({}x{})",
        spatial_raster.width, spatial_raster.height, spatial_raster.height, spatial_raster.width
    );

    // Create symbolic data (text)
    let symbolic = SymbolicData::new("This is a spectrogram of a 440Hz sine wave".to_string());
    println!("SymbolicData: {} chars\n", symbolic.len());

    // Demonstrate the unified type
    use nyx::intermediate::IntermediateData;

    let unified = vec![
        IntermediateData::TimeSig(time_signal),
        IntermediateData::Frequency(frequency_spectrum),
        IntermediateData::Spatial(spatial_raster),
        IntermediateData::Symbolic(symbolic),
    ];

    println!("Unified Intermediate Types:");
    for data in &unified {
        println!("  - {}", data.type_name());
    }
}

#[allow(dead_code)]
fn example_transform_pipeline() {
    use nyx::intermediate::{TimeSignal, IntermediateData};

    println!("\n=== Transform Pipeline (Conceptual) ===\n");

    // Create a simple time signal
    let samples = (0..1000).map(|i| ((i as f32) / 100.0).sin()).collect();
    let signal = TimeSignal::new(samples, 44100);
    let input = IntermediateData::TimeSig(signal);

    println!("Input: {:?}", input.type_name());
    println!("  → [FFT Transformer]");
    println!("  → FrequencySpectrum (complex bins)");
    println!("  ↓");
    println!("  → [Spectrogram Renderer]");
    println!("  → SpatialRaster (PNG pixels)");
    println!("Output: SpatialRaster\n");

    println!("This pipeline is implemented in:");
    println!("  src/transform/audio_to_image.rs → SpatialRaster");
    println!("  src/transform/image_to_audio.rs → TimeSignal (inverse)");
    println!("\nNext step: Adapt these to implement the generic Transform trait.");
}
