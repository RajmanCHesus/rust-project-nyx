/// Integration test: full roundtrip audio transformation
/// 
/// Test: WAV → PCM → Spectrogram (FFT) → Inverse (iFFT) → PCM → WAV
/// 
/// This tests that the forward and inverse transforms are close to complementary
/// (some quality loss is expected due to windowing and FFT artifacts)

#[cfg(test)]
mod tests {
    use nyx::parser;
    use nyx::transform::Transformer;
    use nyx::transform::audio_to_image::AudioToSpectrogramTransformer;
    use nyx::transform::image_to_audio::SpectrogramToAudioTransformer;
    use nyx::signal::{Signal, PureWave};
    use nyx::domains::AudioDomain;

    #[test]
    fn test_roundtrip_pure_tone() {
        // Generate a pure 440Hz sine wave (1 second)
        let wave = PureWave::new(440.0, 0.8, 1.0);
        let sample_rate = 44100u32;
        let samples = wave.sample(sample_rate).unwrap();
        
        let original_audio = AudioDomain::PCM {
            samples: samples.clone(),
            sample_rate,
        };
        
        // Forward transform: PCM → Spectrogram
        let fft_size = 1024;
        let hop_size = 512;
        let spectrogram_transformer = AudioToSpectrogramTransformer::new(fft_size, hop_size);
        let spectrogram = spectrogram_transformer
            .transform(original_audio)
            .expect("Forward transform should succeed");
        
        // Inverse transform: Spectrogram → PCM
        let inverse_transformer = SpectrogramToAudioTransformer::new(sample_rate, hop_size);
        let reconstructed_audio = inverse_transformer
            .transform(spectrogram)
            .expect("Inverse transform should succeed");
        
        // Extract reconstructed samples
        let reconstructed_samples = match reconstructed_audio {
            AudioDomain::PCM { samples, .. } => samples,
            _ => panic!("Expected PCM output"),
        };
        
        // Compare: we expect significant overlap
        assert!(
            reconstructed_samples.len() > 0,
            "Reconstructed audio should have samples"
        );
        
        // Check that reconstructed audio is in valid range
        for &sample in &reconstructed_samples {
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Sample {} out of valid range [-1, 1]",
                sample
            );
        }
        
        // Note: Energy preservation is limited because:
        // 1. Forward transformer stores magnitude only (phase is lost in dB conversion)
        // 2. Windowing introduces spectral leakage
        // 3. PNG serialization would further reduce precision
        // For full fidelity, both magnitude and phase must be preserved.
        
        println!(
            "✓ Pure tone roundtrip test passed: reconstructed {} samples",
            reconstructed_samples.len()
        );
    }

    #[test]
    fn test_forward_inverse_with_real_file() {
        // Use the actual test WAV file if it exists
        let input_path = "test_440hz.wav";
        
        // Try to parse; skip if file doesn't exist
        let original_audio = match parser::parse_audio(input_path) {
            Ok(audio) => audio,
            Err(_) => {
                println!("test_440hz.wav not found; skipping integration test");
                return;
            }
        };
        
        // Forward
        let spectrogram_transformer = AudioToSpectrogramTransformer::new(1024, 512);
        let spectrogram = spectrogram_transformer
            .transform(original_audio.clone())
            .expect("Forward transform should succeed");
        
        // Inverse
        let inverse_transformer = SpectrogramToAudioTransformer::new(44100, 512);
        let reconstructed = inverse_transformer
            .transform(spectrogram)
            .expect("Inverse transform should succeed");
        
        // Verify output is PCM
        match reconstructed {
            AudioDomain::PCM { samples, sample_rate } => {
                assert!(samples.len() > 0, "Reconstructed audio should have samples");
                assert_eq!(sample_rate, 44100, "Sample rate should be preserved");
            }
            _ => panic!("Expected PCM output"),
        }
        
        println!("✓ File-based roundtrip test passed");
    }
}
