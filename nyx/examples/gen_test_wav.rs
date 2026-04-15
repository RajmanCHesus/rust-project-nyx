/// Simple utility to generate test WAV files
use nyx::signal::PureWave;
use nyx::signal::Signal;
use nyx::error::NyxResult;

fn main() -> NyxResult<()> {
    // Generate 440Hz (A4) sine wave for 2 seconds
    let wave = PureWave::new(440.0, 0.8, 200.0);
    let sample_rate = 44100u32;
    
    // Sample the wave
    let samples = wave.sample(sample_rate)?;
    
    // Write to WAV file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create("test_440hz.wav", spec)?;
    for sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let i16_sample = (clamped * i16::MAX as f32) as i16;
        writer.write_sample(i16_sample)?;
    }
    writer.finalize()?;
    
    println!("✓ Generated test_440hz.wav (2 seconds, 440Hz sine wave)");
    println!("  Sample rate: 44.1 kHz");
    println!("  Channels: 1 (mono)");
    println!("  Duration: 2 seconds");
    
    Ok(())
}
