use crate::error::NyxResult;

/// Generic signal trait: represents any signal over time
pub trait Signal {
    /// Get frequency components (Hz) if applicable; None if not frequency-domain
    fn frequencies(&self) -> Option<Vec<f32>>;

    /// Get amplitude at a given time (seconds)
    fn amplitude_at(&self, time: f32) -> f32;

    /// Get total duration in seconds
    fn duration(&self) -> f32;

    /// Generate samples at a given sample rate (Hz)
    fn sample(&self, sample_rate: u32) -> NyxResult<Vec<f32>> {
        let num_samples = (self.duration() * sample_rate as f32).ceil() as usize;
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let time = i as f32 / sample_rate as f32;
            samples.push(self.amplitude_at(time));
        }
        Ok(samples)
    }
}

/// Pure sinusoidal wave
#[derive(Clone, Debug)]
pub struct PureWave {
    pub frequency: f32,   // Hz
    pub amplitude: f32,   // 0.0 to 1.0
    pub duration: f32,    // seconds
    pub phase: f32,       // radians, default 0
}

impl PureWave {
    pub fn new(frequency: f32, amplitude: f32, duration: f32) -> Self {
        PureWave {
            frequency,
            amplitude,
            duration,
            phase: 0.0,
        }
    }

    pub fn with_phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
}

impl Signal for PureWave {
    fn frequencies(&self) -> Option<Vec<f32>> {
        Some(vec![self.frequency])
    }

    fn amplitude_at(&self, time: f32) -> f32 {
        let t = 2.0 * std::f32::consts::PI * self.frequency * time + self.phase;
        self.amplitude * t.sin()
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}

/// PCM signal from raw samples
#[derive(Clone, Debug)]
pub struct PcmSignal {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl PcmSignal {
    pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        PcmSignal { samples, sample_rate }
    }
}

impl Signal for PcmSignal {
    fn frequencies(&self) -> Option<Vec<f32>> {
        None // PCM doesn't inherently represent frequencies
    }

    fn amplitude_at(&self, time: f32) -> f32 {
        let sample_idx = (time * self.sample_rate as f32) as usize;
        if sample_idx < self.samples.len() {
            self.samples[sample_idx]
        } else {
            0.0
        }
    }

    fn duration(&self) -> f32 {
        self.samples.len() as f32 / self.sample_rate as f32
    }

    fn sample(&self, _sample_rate: u32) -> NyxResult<Vec<f32>> {
        // Already sampled; return as-is
        Ok(self.samples.clone())
    }
}

/// Composite signal: sum of multiple signals
pub struct CompositeSignal {
    signals: Vec<Box<dyn SignalCloneable>>,
    duration: f32,
}

impl Clone for CompositeSignal {
    fn clone(&self) -> Self {
        let cloned_signals = self.signals.iter().map(|s| s.box_clone()).collect();
        CompositeSignal {
            signals: cloned_signals,
            duration: self.duration,
        }
    }
}

impl std::fmt::Debug for CompositeSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositeSignal")
            .field("num_signals", &self.signals.len())
            .field("duration", &self.duration)
            .finish()
    }
}

trait SignalCloneable: Signal {
    fn box_clone(&self) -> Box<dyn SignalCloneable>;
}

impl<T: 'static + Signal + Clone> SignalCloneable for T {
    fn box_clone(&self) -> Box<dyn SignalCloneable> {
        Box::new(self.clone())
    }
}

impl CompositeSignal {
    pub fn new(duration: f32) -> Self {
        CompositeSignal {
            signals: Vec::new(),
            duration,
        }
    }

    pub fn add_signal<S: 'static + Signal + Clone>(mut self, signal: S) -> Self {
        self.signals.push(Box::new(signal));
        self
    }
}

impl Signal for CompositeSignal {
    fn frequencies(&self) -> Option<Vec<f32>> {
        let mut freqs = Vec::new();
        for signal in &self.signals {
            if let Some(mut f) = signal.frequencies() {
                freqs.append(&mut f);
            }
        }
        if freqs.is_empty() { None } else { Some(freqs) }
    }

    fn amplitude_at(&self, time: f32) -> f32 {
        self.signals
            .iter()
            .map(|s| s.amplitude_at(time))
            .sum()
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_wave() {
        let wave = PureWave::new(440.0, 0.5, 1.0);
        assert_eq!(wave.duration(), 1.0);
        assert_eq!(wave.frequencies(), Some(vec![440.0]));
    }

    #[test]
    fn test_pcm_signal() {
        let samples = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let pcm = PcmSignal::new(samples.clone(), 44100);
        assert_eq!(pcm.duration(), 5.0 / 44100.0);
        assert_eq!(pcm.amplitude_at(0.0), 0.0);
    }
}
