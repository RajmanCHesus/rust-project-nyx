# PROPOSAL.md
 
## Authors
 
- \[Daniel Damek\]
- \[Jakub Chmura\]
 
---
 
## Introduction
 
Modern multimedia data exists in fundamentally separate silos — audio lives in waveforms, images in pixel grids, and text in symbol sequences. Yet human perception constantly crosses these boundaries: we imagine sounds when we see images, we visualize music, we hear words in our heads when we read. This project builds a **cross-modal transformation system** in Rust that bridges these silos programmatically.
 
The core idea is a CLI tool and library that can transform data across modalities: audio into images, images into audio, and text into audio — using signal processing, spectral analysis, and data modeling techniques rather than machine learning.
 
**What problem does it solve?**
There is no simple, composable, open-source Rust tool that treats multimedia files as interchangeable signal data and allows lossless-enough transformation between them. This project explores what it means to encode one modality's information into another's representation space.
 
**What do we hope to learn?**
- How digital signals (PCM audio, pixel data) relate mathematically, and how to exploit those relationships for cross-modal mapping
- How to design a clean, extensible Rust architecture around file parsing, signal processing pipelines, and rendering backends
- How to manage real-world performance constraints when operating on large binary data in Rust (zero-copy parsing, iterator-based streaming, FFT optimization)
- The practical tradeoffs between fidelity and perceptual plausibility when transforming between modalities
 
---
 
## Requirements
 
The following capabilities are required for the project to be considered successful:
 
### Core Transformations
- **Audio → Image**: Convert an audio file (PCM/MP3) into a visual representation (PNG/JPG) — e.g., via spectrogram generation using Short-Time Fourier Transform (STFT), mapping frequency bins to pixel rows and time frames to columns
- **Image → Audio**: Convert an image (PNG/JPG) into an audio file (WAV/MP3) — e.g., by interpreting pixel rows as frequency components and synthesizing audio via inverse FFT
- **Text → Audio**: Convert a plain text string into an audio file — e.g., via a basic additive synthesis approach where character/phoneme values modulate frequency and amplitude
 
### File I/O
- Parse raw PCM and MP3 audio into in-memory signal buffers
- Read and decode PNG/JPG images into pixel buffers
- Write output files in the appropriate target format (WAV, PNG)
 
### CLI Interface
- Accept transformation mode, input file, and output file as CLI arguments
- Provide useful error messages and usage instructions
- Support optional flags for tuning parameters (e.g., FFT window size, sample rate, image resolution)
 
### Architecture
- Clean separation between: file parsing, signal processing, data modeling, and rendering/output stages
- Modular pipeline design so each stage can be tested independently
 
### Performance
- Handle at minimum 30 seconds of audio or a 1024×1024 image without excessive memory usage
- FFT operations should complete in reasonable time (< 10 seconds on a modern laptop for above inputs)
 
---
 
## Dependencies
 
| Crate | Purpose | Link |
|---|---|---|
| `hound` | WAV file reading and writing (PCM audio I/O) | [lib.rs/hound](https://lib.rs/crates/hound) |
| `rodio` | Audio decoding (MP3 support) and playback | [lib.rs/rodio](https://lib.rs/crates/rodio) |
| `rustfft` | Fast Fourier Transform — core of the signal processing pipeline | [lib.rs/rustfft](https://lib.rs/crates/rustfft) |
| `image` | Image decoding/encoding (PNG, JPG) and pixel buffer manipulation | [lib.rs/image](https://lib.rs/crates/image) |
| `clap` | CLI argument parsing with derive macros | [lib.rs/clap](https://lib.rs/crates/clap) |
| `anyhow` | Ergonomic error handling and propagation across pipeline stages | [lib.rs/anyhow](https://lib.rs/crates/anyhow) |
 
Possible additional dependencies (to be evaluated):
- `apodize` — windowing functions (Hann, Hamming) for STFT to reduce spectral leakage
- `rayon` — data parallelism for FFT frame processing if performance requires it
 
---
 
## Architecture
 
The system is organized as a pipeline with four clearly separated layers:
 
```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI  (clap)                              │
│         mode: audio→image | image→audio | text→audio           │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                     FILE PARSING LAYER                          │
│                                                                 │
│   ┌──────────────┐   ┌──────────────┐   ┌──────────────────┐  │
│   │  Audio Parser│   │ Image Parser │   │   Text Parser    │  │
│   │  hound/rodio │   │    image     │   │   std::string    │  │
│   │  → PCM f32[] │   │  → Rgb8 buf  │   │   → char vec     │  │
│   └──────┬───────┘   └──────┬───────┘   └────────┬─────────┘  │
└──────────┼─────────────────┼───────────────────── ┼────────────┘
           │                 │                       │
           ▼                 ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                  SIGNAL PROCESSING LAYER                        │
│                                                                 │
│   ┌──────────────────────┐     ┌────────────────────────────┐  │
│   │  Spectral Analysis   │     │   Synthesis Engine         │  │
│   │  rustfft STFT        │     │   Inverse FFT / Additive   │  │
│   │  → Complex<f32>[][]  │     │   → PCM f32[]              │  │
│   └──────────────────────┘     └────────────────────────────┘  │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    DATA MODELING LAYER                          │
│                                                                 │
│   Normalization · Frequency-to-Pixel mapping                   │
│   Amplitude-to-Color mapping · Colormap application            │
│   Time-frame ↔ Column index alignment                          │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                     RENDERING LAYER                             │
│                                                                 │
│   ┌──────────────────┐         ┌──────────────────────────┐   │
│   │  Image Renderer  │         │     Audio Renderer       │   │
│   │  image → PNG/JPG │         │     hound → WAV          │   │
│   └──────────────────┘         └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```
 
### Transformation Flows
 
```
audio→image:  PCM samples → STFT → magnitude spectrogram → colormap → PNG
image→audio:  pixel rows → frequency bins → inverse FFT → PCM → WAV
text→audio:   characters → frequency/amplitude values → additive synthesis → PCM → WAV
```
 
---
