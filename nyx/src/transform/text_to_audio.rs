use crate::domains::{TextDomain, AudioDomain};
use crate::error::NyxResult;
use crate::transform::Transformer;

/// Transform text to audio using additive synthesis
pub struct TextToAudioTransformer {
    pub sample_rate: u32,
    pub char_duration: f32, // Duration of each character in seconds
    pub base_freq: f32,     // Base frequency for character mapping
}

impl TextToAudioTransformer {
    pub fn new(sample_rate: u32, char_duration: f32, base_freq: f32) -> Self {
        Self {
            sample_rate,
            char_duration,
            base_freq,
        }
    }

    pub fn default() -> Self {
        Self {
            sample_rate: 44100,
            char_duration: 0.1,
            base_freq: 220.0,
        }
    }

    /// Map a character to a specific frequency (Hz)
    fn char_to_freq(&self, c: char) -> f32 {
        let char_code = c as u32;
        // Map ASCII printable characters to a semitone shift from base frequency
        let semitone = (char_code.saturating_sub(32) % 64) as f32;
        self.base_freq * 2.0_f32.powf(semitone / 12.0)
    }

    /// Map a character to an amplitude
    fn char_to_amplitude(&self, c: char) -> f32 {
        if c.is_whitespace() {
            0.0 // Silence for spaces
        } else if c.is_uppercase() {
            0.8 // Louder for uppercase
        } else if c.is_ascii_punctuation() {
            0.4 // Quieter for punctuation
        } else {
            0.6 // Normal volume for lowercase
        }
    }
}

impl Transformer<TextDomain, AudioDomain> for TextToAudioTransformer {
    fn transform(&self, input: TextDomain) -> NyxResult<AudioDomain> {
        match input {
            TextDomain::Plain(text) => {
                let samples_per_char = (self.char_duration * self.sample_rate as f32) as usize;
                let mut pcm_samples = Vec::new();

                // characters → frequency/amplitude values → additive synthesis → PCM
                for c in text.chars() {
                    let freq = self.char_to_freq(c);
                    let amp = self.char_to_amplitude(c);

                    for i in 0..samples_per_char {
                        let t = i as f32 / self.sample_rate as f32;
                        // Additive synthesis (basic sine wave)
                        let sample = amp * (2.0 * std::f32::consts::PI * freq * t).sin();

                        // Simple linear envelope to prevent clicking artifacts
                        let fade_len = (self.sample_rate as f32 * 0.01) as usize; // 10ms fade
                        let fade_len = fade_len.min(samples_per_char / 2);

                        let envelope = if i < fade_len {
                            i as f32 / fade_len as f32
                        } else if i > samples_per_char - fade_len {
                            (samples_per_char - i) as f32 / fade_len as f32
                        } else {
                            1.0
                        };

                        pcm_samples.push(sample * envelope);
                    }
                }

                Ok(AudioDomain::PCM {
                    samples: pcm_samples,
                    sample_rate: self.sample_rate,
                })
            }
        }
    }
}
