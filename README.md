<!--[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/RHGu4AQi)-->

# nyx — Cross-Modal Transformation System

**A Rust system for bidirectional audio ↔ image transformations using FFT-based spectrograms.**

## Status: Phase 3b Complete ✓

- ✓ Audio → Spectrogram (visualization via FFT)
- ✓ Spectrogram → Audio (reconstruction via inverse FFT)
- ✓ CLI with configurable FFT parameters
- ✓ Full test coverage (unit + integration tests)

## Quick Start

```bash
cd nyx

# Build
cargo build --release

# Generate test audio
cargo run --example gen_test_wav
# → Creates: test_440hz.wav (440Hz sine, 2s, 44.1kHz)

# Transform audio to spectrogram
./target/release/nyx test_440hz.wav spectrum.png
# → Creates: spectrum.png (513×173, shows 440Hz band)

# Transform spectrogram back to audio
./target/release/nyx spectrum.png reconstructed.wav --mode image-to-audio
# → Creates: reconstructed.wav (PCM 16-bit)
```

## Architecture

### Layer Model (Parser → Domain → Transform → Render)

```
InputFile (WAV/PNG)
    ↓
[Parser] — hound, image crates
    ↓
Domain (PCM, Frequency Matrix, Pixels)
    ↓
[Transformer] — FFT, iFFT, algorithms
    ↓
[Renderer] — WAV output, PNG spectrogram
    ↓
OutputFile
```

### Transformers

1. **Audio → Image** (spectrogram visualization)
   - STFT + Hann windowing + FFT
   - Magnitude → dB log scale → PNG grayscale
   
2. **Image → Audio** (reconstruction)
   - PNG → frequency matrix
   - Inverse FFT with Hermitian symmetry
   - Overlap-add windowing → PCM output

## Implementation Details

### Supported Formats

- **Audio**: WAV (PCM 16-bit) at any sample rate
- **Image**: PNG (8-bit RGB) at any resolution
- **Module**: Text (UTF-8, for future expansion)

### Core Components

- `Signal` trait — generic signal representation (PureWave, PcmSignal)
- `Matrix<T>` — generic 2D container (frequency bins × time, width × height)
- `Transformer<I, O>` — trait for domain-to-domain transformations
- `Renderer<I>` — trait for output serialization
- `NyxError` — unified error handling with context

### Dependencies

| Crate | Purpose |
|-------|---------|
| `hound` | WAV file I/O |
| `rustfft` | Fast Fourier Transform |
| `image` | PNG image I/O |
| `clap` | CLI parsing |
| `num-complex` | Complex number support |

## Results

### Tested Transformations

✓ **Pure tone (440Hz)** — 2-second sine wave
  - Audio → Spectrogram: Bright band at 440Hz ✓
  - Spectrogram → Audio: Reconstructs PCM ✓

✓ **All tests passing** (6 unit + 2 integration)

✓ **Zero compiler warnings**

## Known Limitations

### Phase Preservation
- PNG spectrogram stores **magnitude only** (phase lost in log conversion)
- Roundtrip Audio→PNG→Audio has ~83% energy loss (documented)
- **Fix**: Store phase separately or use binary format

### Deferred Features
- MP3/FLAC decoding (add `symphonia` crate)
- Audio playback (add `rodio` crate)
- Real-time streaming
- Multi-channel audio (currently mono only)
- Image→Audio visual frequency mapping

## Development Phases Completed

| Phase | Topic | Status |
|-------|-------|--------|
| 0 | Scope definition | ✓ |
| 1 | Rust foundations | ✓ (reading) |
| 2 | Architecture design | ✓ |
| 3 | Audio→Spectrogram | ✓ |
| 3b | Bidirectional roundtrip | ✓ |
| 4 | File format reference | ✓ |
| 5 | Crates integration | ✓ |

## Documentation

- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) — Full architecture + roadmap
- [FILE_FORMATS.md](FILE_FORMATS.md) — WAV/PNG/UTF-8 format reference
- Inline comments in source code (minimal but comprehensive)

## Next Steps

1. **MP3 Support** — Add `symphonia` for compressed audio
2. **Playback** — Add `rodio` for real-time audio output
3. **Phase Preservation** — Binary format for full-fidelity roundtrip
4. **Performance** — Benchmarks + optimization
5. **Visual Mapping** — Pixel coordinates → frequency synthesis

## Build

```bash
cargo build --release      # ~25s first build
cargo test --release       # All tests pass
cargo run -- --help        # CLI help
```

**Binary**: `target/release/nyx` (~6.2 MB unstripped)

---

*nyx* — Built from first principles, no ffmpeg wrapping, pure Rust DSP.  
