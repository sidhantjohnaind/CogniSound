<div align="center">

# 🧠 CogniSound — Cognitive AI Music Player & Parallel Audio Suite

**An ultra-high-performance, parallel audio engine, neural music intelligence suite, and modern web player written in 100% native Rust.**

[![Rust 2024](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Web_Framework-Axum_0.8-blue?logo=tokio)](https://github.com/tokio-rs/axum)
[![Tokio](https://img.shields.io/badge/Async_Runtime-Tokio_1.43-8A2BE2?logo=tokio)](https://tokio.rs/)
[![SQLite WAL](https://img.shields.io/badge/Database-SQLite_WAL-003B57?logo=sqlite)](https://www.sqlite.org/)
[![Audio Engine](https://img.shields.io/badge/Audio_Engine-WASAPI_Exclusive_%2B_CPAL-green)](#-audio-engineering--dsp-suite)
[![GPU Accelerated](https://img.shields.io/badge/ML_Inference-DirectML_%2F_CUDA-76B900?logo=nvidia)](#-ai--dsp-music-intelligence)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[Features](#-key-features) • [Architecture](#-architecture--parallel-engine) • [Quickstart](#-quickstart) • [CLI Suite](#-cli-tools-reference) • [API Reference](#-api-endpoints)

</div>

---

## ⚡ Key Features

- **🚀 Extreme Concurrency**: Multi-threaded metadata extraction, streaming SQLite transactions, and lock-free Rayon work-stealing thread pools.
- **🔒 Bit-Perfect WASAPI Exclusive Engine**: True hardware DAC clock matching, 0 dB digital volume bypass (bypassing Windows `audiodg.exe`), linear resampling, and real-time 10-band parametric EQ.
- **🧠 Audio DSP & AI Intelligence**:
  - **PANNs CNN14 & YAMNet ML**: Hardware-accelerated (DirectML/CUDA) neural audio classification.
  - **Spectral Stem Analysis**: Multi-band FFT power analysis separating bass, vocal, drums, and harmonic energy.
  - **Melody Twin Calculator**: Sub-second acoustic similarity matching (**2,300+ tracks analyzed in ~120ms**).
  - **Harmonic DJ & Auto-DJ**: Camelot wheel harmonic mixing and BPM-matched queueing.
- **📱 Responsive Web Interface & Remote**:
  - PWA dashboard with real-time WebSocket telemetry.
  - Advanced drilldown filtering by artist consensus, decade, genre category, emotion, and musical key.
  - Mobile LAN remote control with QR code pairing.
- **📦 100% Native Rust**: Zero runtime dependencies, standalone binary execution, and minimal memory footprint (~40MB RAM).

---

## 🏗️ Architecture & Parallel Engine

Sonar separates time-critical playback from intensive background intelligence tasks to guarantee stutter-free, zero-latency listening:

```
                         ┌────────────────────────────────────────┐
                         │       Web UI / PWA / Remote            │
                         └──────────────────┬─────────────────────┘
                                            │ HTTP / WebSockets
                                            ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           Native Rust Server (Axum + Tokio)                     │
│                                                                                 │
│  ┌───────────────────────┐  ┌───────────────────────┐  ┌─────────────────────┐  │
│  │   Tokio Async I/O     │  │   Rayon Thread Pool   │  │ WASAPI / CPAL Engine│  │
│  │  (REST API & WS Feed) │  │  (Adaptive 75% Budget)│  │ (Bit-Perfect Play)  │  │
│  └───────────┬───────────┘  └───────────┬───────────┘  └──────────┬──────────┘  │
└──────────────┼──────────────────────────┼─────────────────────────┼─────────────┘
               │                          │                         │
               ▼                          ▼                         ▼
┌───────────────────────────┐ ┌───────────────────────┐ ┌─────────────────────────┐
│ SQLite WAL Database Pool  │ │ Parallel CLI Binaries │ │  Hardware Audio Output  │
│ (Streaming 500-item batch)│ │ (Scanner, ML, Stems)  │ │ (Direct DAC Exclusive)  │
└───────────────────────────┘ └───────────────────────┘ └─────────────────────────┘
```

### 📊 Performance Benchmarks

| Subsystem / Operation | Python Baseline | Sonar Parallel Rust Engine | Speedup Factor | Memory Footprint |
| :--- | :--- | :--- | :--- | :--- |
| **Melody Twin Calculation** (`compute_melody_twins`) | ~15 – 30 seconds | **120.82 ms** | 🚀 **~200x Faster** | < 15 MB |
| **Library DB Scanner** (`db_scanner` - 2,298 tracks) | ~3 – 5 minutes | **40.03 seconds** | ⚡ **~6x Faster** | ~40 MB |
| **Spectral Stem Analyzer** (`stem_analyzer` - STFT) | ~5.0s / track | **~0.70s / track** | ⚡ **~8x Faster** | ~35 MB |
| **Junction Table Population** (`generate_timelines`) | ~12 seconds | **~100 ms** | 🚀 **~120x Faster** | < 10 MB |
| **ML Audio Classification** (`classify_tracks` DirectML) | ~2.0s / track | **~0.50s / track** | ⚡ **~4x Faster** | ~85 MB |
| **Web Server Baseline RAM** (`rust_server` vs Python) | ~450 MB – 850 MB | **~38 MB – 55 MB** | 📉 **15x Less RAM** | — |

---

## 🚀 Quickstart

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (2024 Edition / 1.85+)
- Windows 10/11 (WASAPI Exclusive + DirectML support) or Linux (ALSA/PulseAudio)

### 1. Launch with One Click (Windows)
Double-click `play_browser.bat`. It will automatically build release binaries if not yet compiled and launch the server on port `80`:
```cmd
play_browser.bat
```

### 2. Launch on Linux
```bash
chmod +x play_browser.sh
./play_browser.sh
```

### 3. Build From Source
```bash
# Build all release binaries
cargo build --release --manifest-path rust_server/Cargo.toml

# Start the web server
cargo run --release --manifest-path rust_server/Cargo.toml --bin rust_server
```

### 4. Multi-Architecture Compilation (AMD64, ARM64, RISC-V)
Sonar supports cross-compilation across multiple hardware architectures:

| Architecture | Target Triple | Windows / Linux |
| :--- | :--- | :--- |
| **AMD64 (x86_64)** | `x86_64-pc-windows-msvc` / `x86_64-unknown-linux-gnu` | `cargo build --release --target <triple>` |
| **ARM64 (aarch64)** | `aarch64-pc-windows-msvc` / `aarch64-unknown-linux-gnu` | `cross build --release --target <triple>` |
| **RISC-V (riscv64)** | `riscv64gc-unknown-linux-gnu` | `cross build --release --target <triple>` |

You can also use the automated multi-arch build scripts:
- **Windows (PowerShell)**: `.\build_all_arch.ps1`
- **Linux/macOS (Bash)**: `./build_all_arch.sh`
- **GitHub CI**: Pre-configured multi-platform build matrix in `.github/workflows/build-all-arch.yml`.

---

## 🛠️ CLI Tools Reference

Sonar includes dedicated, parallel CLI binaries for library management and batch processing:

| Binary | Description | Usage Example |
| :--- | :--- | :--- |
| `rust_server` | Main Axum web server, playback engine & WebSocket broadcaster | `cargo run --release --bin rust_server` |
| `db_scanner` | Multi-threaded file walker & tag/LRC metadata synchronizer | `cargo run --release --bin db_scanner -- --threads 12` |
| `stem_analyzer` | Parallel STFT spectral power & frequency band analyzer | `cargo run --release --bin stem_analyzer -- --limit 50` |
| `classify_tracks` | DirectML/CUDA GPU accelerated YAMNet/CNN14 ML classifier | `cargo run --release --bin classify_tracks -- --pending-only` |
| `compute_melody_twins` | Multi-threaded vector similarity & acoustic twin engine | `cargo run --release --bin compute_melody_twins` |
| `generate_timelines` | Framewise instrument presence timeline generator | `cargo run --release --bin generate_timelines` |

---

## 🌐 API Endpoints

### Track & Library Management
- `GET /api/tracks?search={query}&page={n}&limit={m}` — Full library search & drilldown.
- `GET /api/track?id={id}` — Single track detailed intelligence & metadata.
- `GET /api/track/lyrics?id={id}` — Synced timestamped LRC / plain text lyrics.
- `POST /api/track/update_tags` — Edit title, artist, album, and custom tags.

### Playback & DSP Control
- `GET /api/playback/status` — Real-time transport status (active track, time offset, state).
- `POST /api/playback/play` — Load and start bit-perfect WASAPI playback.
- `POST /api/playback/pause` / `POST /api/playback/resume` — Transport controls.
- `POST /api/playback/seek` — Zero-latency audio buffer seek (`{ position_ms: 12000 }`).
- `POST /api/dsp/eq` — Update 10-band parametric equalizer gains in real time.

### Diagnostics & Real-Time Telemetry
- `GET /api/system/diagnostics` — Library overview statistics, top artists, key distributions.
- `GET /ws` — Low-latency WebSocket broadcasting transport progress and queue state.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
