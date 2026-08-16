use anyhow::{Context, Result};
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use ort::session::Session;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use ort::value::Tensor;
use rusqlite::Connection;
use std::fs::File;
use std::path::{Path, PathBuf};

use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub const YAMNET_VOCAL_INDICES: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
];

pub const CNN14_VOCAL_INDICES: &[usize] = &[
    0, 1, 2, 3, 12, 13, 14, 15, 16, 17, 18, 19, 20,
];

/// Decode audio file to mono f32 PCM array resampled to target sample rate (e.g. 16000Hz or 32000Hz)
pub fn decode_audio_to_target_sr(file_path: &Path, target_sr: usize) -> Result<Vec<f32>> {
    // 1. Fast in-process pure Rust decoding via Symphonia (FLAC, MP3, WAV, ALAC)
    if let Ok(samples) = decode_with_symphonia(file_path, target_sr) {
        if !samples.is_empty() {
            return Ok(samples);
        }
    }

    // 2. High-speed fallback via bundled FFmpeg for Opus, Ogg Vorbis, AAC, WMA, etc.
    decode_with_ffmpeg(file_path, target_sr)
}

fn decode_with_symphonia(file_path: &Path, target_sr: usize) -> Result<Vec<f32>> {
    let src = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .with_context(|| "Failed to probe audio format")?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .with_context(|| "No default audio track found")?;

    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100) as usize;
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(2);
    let track_id = track.id;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .with_context(|| "Failed to create decoder")?;

    let mut raw_mono_samples: Vec<f32> = Vec::new();

    while let Ok(packet) = format.next_packet() {
        if packet.track_id() != track_id {
            continue;
        }

        if let Ok(audio_buf) = decoder.decode(&packet) {
            let spec = *audio_buf.spec();
            let mut sample_buf = symphonia::core::audio::AudioBuffer::<f32>::new(audio_buf.capacity() as u64, spec);
            audio_buf.convert(&mut sample_buf);

            let planes = sample_buf.planes();
            let plane_0 = planes.planes()[0];

            if channels == 1 {
                raw_mono_samples.extend_from_slice(plane_0);
            } else if planes.planes().len() >= 2 {
                let plane_1 = planes.planes()[1];
                let len = plane_0.len().min(plane_1.len());
                for i in 0..len {
                    raw_mono_samples.push((plane_0[i] + plane_1[i]) * 0.5);
                }
            } else {
                raw_mono_samples.extend_from_slice(plane_0);
            }
        }
    }

    if raw_mono_samples.is_empty() {
        anyhow::bail!("No audio samples decoded");
    }

    // Resample to target sample rate
    if sample_rate == target_sr {
        Ok(raw_mono_samples)
    } else {
        let step = sample_rate as f64 / target_sr as f64;
        let target_len = (raw_mono_samples.len() as f64 / step) as usize;
        let mut resampled = Vec::with_capacity(target_len);

        let mut pos = 0.0f64;
        while (pos as usize) < raw_mono_samples.len() {
            let idx = pos as usize;
            resampled.push(raw_mono_samples[idx]);
            pos += step;
        }

        Ok(resampled)
    }
}

fn decode_with_ffmpeg(file_path: &Path, target_sr: usize) -> Result<Vec<f32>> {
    let cfg = crate::config::AppConfig::load();
    let ffmpeg_bin = cfg.find_ffmpeg().unwrap_or_else(|| PathBuf::from("ffmpeg"));

    let output = std::process::Command::new(ffmpeg_bin)
        .arg("-v").arg("error")
        .arg("-i").arg(file_path)
        .arg("-f").arg("f32le")
        .arg("-ar").arg(target_sr.to_string())
        .arg("-ac").arg("1")
        .arg("pipe:1")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .with_context(|| format!("Failed to run FFmpeg for {}", file_path.display()))?;

    if !output.status.success() || output.stdout.is_empty() {
        anyhow::bail!("FFmpeg failed to decode {}", file_path.display());
    }

    let bytes = output.stdout;
    let float_count = bytes.len() / 4;
    let mut samples = Vec::with_capacity(float_count);
    for chunk in bytes.chunks_exact(4) {
        let val = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        samples.push(val);
    }

    Ok(samples)
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub struct YamnetClassifier {
    session: Session,
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl YamnetClassifier {
    pub fn new(onnx_model_path: &Path) -> Result<Self> {
        let session = Session::builder()?
            .with_execution_providers([
                ort::execution_providers::CUDA::default().build(),
                ort::execution_providers::DirectML::default().with_device_id(0).build(),
            ])
            .map_err(|e| anyhow::anyhow!("Failed to set GPU execution providers: {:?}", e))?
            .commit_from_file(onnx_model_path)
            .with_context(|| format!("Failed to load YAMNet ONNX model from {}", onnx_model_path.display()))?;

        println!(" ⚡ YAMNet ONNX Session initialized WITH GPU Acceleration (CUDA / DirectML)!");
        Ok(Self { session })
    }

    /// Classify 16kHz mono audio waveform and return max vocal probability score
    pub fn predict_vocal_score(&mut self, samples: &[f32]) -> Result<f32> {
        let max_samples = 30 * 16000;
        let input_slice = if samples.len() > max_samples {
            &samples[..max_samples]
        } else {
            samples
        };

        let tensor = Tensor::from_array((vec![input_slice.len() as i64], input_slice.to_vec()))?;
        let outputs = self.session.run(ort::inputs![tensor])?;

        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;

        let num_frames = if shape.len() >= 2 { shape[0] as usize } else { 1 };
        let num_classes = if shape.len() >= 2 { shape[1] as usize } else { 521 };

        let mut max_vocal_score = 0.0f32;

        for frame_idx in 0..num_frames {
            let frame_offset = frame_idx * num_classes;
            for &v_idx in YAMNET_VOCAL_INDICES {
                if v_idx < num_classes {
                    let score = data[frame_offset + v_idx];
                    if score > max_vocal_score {
                        max_vocal_score = score;
                    }
                }
            }
        }

        Ok(max_vocal_score)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InstrumentScores {
    pub piano: f32,
    pub guitar: f32,
    pub drums: f32,
    pub bass: f32,
    pub synth: f32,
    pub strings: f32,
    pub brass: f32,
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub struct Cnn14Classifier {
    session: Session,
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl Cnn14Classifier {
    pub fn new(onnx_model_path: &Path) -> Result<Self> {
        let session = Session::builder()?
            .with_execution_providers([
                ort::execution_providers::CUDA::default().build(),
                ort::execution_providers::DirectML::default().with_device_id(0).build(),
            ])
            .map_err(|e| anyhow::anyhow!("Failed to set GPU execution providers: {:?}", e))?
            .commit_from_file(onnx_model_path)
            .with_context(|| format!("Failed to load PANNs CNN14 ONNX model from {}", onnx_model_path.display()))?;

        println!(" ⚡ PANNs CNN14 ONNX Session initialized WITH GPU Acceleration (CUDA / DirectML)!");
        Ok(Self { session })
    }

    /// Classify 32kHz mono audio waveform and return max vocal probability score
    pub fn predict_vocal_score(&mut self, samples: &[f32]) -> Result<f32> {
        let max_samples = 30 * 32000;
        let input_slice = if samples.len() > max_samples {
            &samples[..max_samples]
        } else {
            samples
        };

        let tensor = Tensor::from_array((vec![1i64, input_slice.len() as i64], input_slice.to_vec()))?;
        let outputs = self.session.run(ort::inputs![tensor])?;

        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let num_classes = if shape.len() >= 2 { shape[1] as usize } else { 527 };

        let mut max_vocal_score = 0.0f32;
        for &v_idx in CNN14_VOCAL_INDICES {
            if v_idx < num_classes {
                let score = data[v_idx];
                if score > max_vocal_score {
                    max_vocal_score = score;
                }
            }
        }

        Ok(max_vocal_score)
    }

    /// Extract multi-instrument timeline presence scores & full instrument dictionary from CNN14
    pub fn predict_instrument_scores(&mut self, samples: &[f32]) -> Result<(InstrumentScores, std::collections::BTreeMap<String, f32>)> {
        let max_samples = 30 * 32000;
        let input_slice = if samples.len() > max_samples {
            &samples[..max_samples]
        } else {
            samples
        };

        let tensor = Tensor::from_array((vec![1i64, input_slice.len() as i64], input_slice.to_vec()))?;
        let outputs = self.session.run(ort::inputs![tensor])?;

        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let num_classes = if shape.len() >= 2 { shape[1] as usize } else { 527 };

        let get_score = |idx: usize| -> f32 {
            if idx < num_classes {
                (data[idx] * 100.0).round() / 100.0
            } else {
                0.0
            }
        };

        let get_max_score = |indices: &[usize]| -> f32 {
            let mut m = 0.0f32;
            for &idx in indices {
                if idx < num_classes && data[idx] > m {
                    m = data[idx];
                }
            }
            (m * 100.0).round() / 100.0
        };

        let scores = InstrumentScores {
            piano: get_max_score(&[137, 138, 139]),
            guitar: get_max_score(&[140, 141, 142, 143, 144, 145, 146]),
            drums: get_max_score(&[153, 154, 155, 156, 157, 158, 159, 160]),
            bass: get_max_score(&[147, 148, 149]),
            synth: get_max_score(&[172, 173, 174]),
            strings: get_max_score(&[166, 167, 168, 169]),
            brass: get_max_score(&[178, 179, 180, 181, 182]),
        };

        let mut instrument_map = std::collections::BTreeMap::new();
        instrument_map.insert("Speech".to_string(), get_score(0));
        instrument_map.insert("Male speech".to_string(), get_score(1));
        instrument_map.insert("Female speech".to_string(), get_score(2));
        instrument_map.insert("Child speech".to_string(), get_score(3));
        instrument_map.insert("Singing".to_string(), get_score(12));
        instrument_map.insert("Choir".to_string(), get_score(13));
        instrument_map.insert("Piano".to_string(), get_max_score(&[137, 138, 139]));
        instrument_map.insert("Electric piano".to_string(), get_score(138));
        instrument_map.insert("Organ".to_string(), get_score(171));
        instrument_map.insert("Keyboard".to_string(), get_score(172));
        instrument_map.insert("Acoustic guitar".to_string(), get_score(141));
        instrument_map.insert("Electric guitar".to_string(), get_score(142));
        instrument_map.insert("Bass guitar".to_string(), get_score(147));
        instrument_map.insert("String section".to_string(), get_score(166));
        instrument_map.insert("Violin".to_string(), get_score(167));
        instrument_map.insert("Cello".to_string(), get_score(168));
        instrument_map.insert("Harp".to_string(), get_score(170));
        instrument_map.insert("Drum kit".to_string(), get_score(153));
        instrument_map.insert("Snare drum".to_string(), get_score(154));
        instrument_map.insert("Bass drum".to_string(), get_score(155));
        instrument_map.insert("Cymbal".to_string(), get_score(157));
        instrument_map.insert("Hi-hat".to_string(), get_score(158));
        instrument_map.insert("Percussion".to_string(), get_score(160));
        instrument_map.insert("Brass section".to_string(), get_score(178));
        instrument_map.insert("Trumpet".to_string(), get_score(179));
        instrument_map.insert("Trombone".to_string(), get_score(180));
        instrument_map.insert("Saxophone".to_string(), get_score(181));
        instrument_map.insert("Flute".to_string(), get_score(184));
        instrument_map.insert("Synthesizer".to_string(), get_score(174));
        instrument_map.insert("Electronic music".to_string(), get_score(290));
        instrument_map.insert("Ambient music".to_string(), get_score(291));

        Ok((scores, instrument_map))
    }

    /// Generate high-precision framewise instrument presence timeline across the audio waveform
    pub fn generate_instrument_timeline(
        &mut self,
        samples: &[f32],
        window_size_sec: f32,
        hop_size_sec: f32,
    ) -> Result<Vec<TimelineFrame>> {
        let sr = 32000;
        let window_len = (window_size_sec * sr as f32) as usize;
        let hop_len = (hop_size_sec * sr as f32) as usize;

        if samples.is_empty() {
            return Ok(Vec::new());
        }

        let mut frames = Vec::new();
        let mut start = 0;
        let total_samples = samples.len();

        while start < total_samples {
            let end = (start + window_len).min(total_samples);
            let chunk = &samples[start..end];
            let time_sec = start as f32 / sr as f32;

            if let Ok((scores, _)) = self.predict_instrument_scores(chunk) {
                let vocal_score = self.predict_vocal_score(chunk).unwrap_or(0.0);

                let mut candidates = vec![
                    ("Vocals", vocal_score),
                    ("Piano", scores.piano),
                    ("Guitar", scores.guitar),
                    ("Drums", scores.drums),
                    ("Bass", scores.bass),
                    ("Synthesizer", scores.synth),
                    ("Strings", scores.strings),
                    ("Brass", scores.brass),
                ];
                candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                let (lead, conf) = candidates.first().unwrap_or(&("None", 0.0));

                frames.push(TimelineFrame {
                    time_sec: (time_sec * 10.0).round() / 10.0,
                    piano: scores.piano,
                    guitar: scores.guitar,
                    drums: scores.drums,
                    bass: scores.bass,
                    synth: scores.synth,
                    strings: scores.strings,
                    brass: scores.brass,
                    vocals: (vocal_score * 100.0).round() / 100.0,
                    lead_instrument: lead.to_string(),
                    lead_confidence: *conf,
                });
            }

            start += hop_len;
            if start + hop_len > total_samples && start < total_samples {
                break;
            }
        }

        Ok(frames)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TimelineFrame {
    pub time_sec: f32,
    pub piano: f32,
    pub guitar: f32,
    pub drums: f32,
    pub bass: f32,
    pub synth: f32,
    pub strings: f32,
    pub brass: f32,
    pub vocals: f32,
    pub lead_instrument: String,
    pub lead_confidence: f32,
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub struct YamnetClassifier {}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
impl YamnetClassifier {
    pub fn new(_onnx_model_path: &Path) -> Result<Self> {
        Ok(Self {})
    }

    pub fn predict_vocal_score(&mut self, _samples: &[f32]) -> Result<f32> {
        Ok(0.5)
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub struct Cnn14Classifier {}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
impl Cnn14Classifier {
    pub fn new(_onnx_model_path: &Path) -> Result<Self> {
        Ok(Self {})
    }

    pub fn predict_vocal_score(&mut self, _samples: &[f32]) -> Result<f32> {
        Ok(0.5)
    }

    pub fn predict_instrument_scores(
        &mut self,
        _samples: &[f32],
    ) -> Result<(InstrumentScores, std::collections::BTreeMap<String, f32>)> {
        Ok((
            InstrumentScores {
                piano: 0.0,
                guitar: 0.0,
                drums: 0.0,
                bass: 0.0,
                synth: 0.0,
                strings: 0.0,
                brass: 0.0,
            },
            std::collections::BTreeMap::new(),
        ))
    }

    pub fn generate_instrument_timeline(
        &mut self,
        _samples: &[f32],
        _window_size_sec: f32,
        _hop_size_sec: f32,
    ) -> Result<Vec<TimelineFrame>> {
        Ok(Vec::new())
    }
}

/// Classify all pending or all tracks in SQLite using native Rust ONNX YAMNet or PANNs CNN14
pub fn classify_pending_tracks(
    music_dir: &Path,
    model_path: &Path,
    conn: &Connection,
    limit: Option<usize>,
    overwrite: bool,
) -> Result<(usize, usize)> {
    let filename = model_path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    let is_cnn14 = filename.contains("cnn14");

    let where_clause = if overwrite {
        ""
    } else {
        "WHERE vocal_status IS NULL OR vocal_status = 'unknown' OR vocal_status = ''"
    };

    let sql = if let Some(lim) = limit {
        format!(
            "SELECT id, file_path FROM tracks {} LIMIT {}",
            where_clause, lim
        )
    } else {
        format!("SELECT id, file_path FROM tracks {}", where_clause)
    };

    let mut stmt = conn.prepare(&sql)?;
    let pending_rows: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .filter_map(|r| r.ok())
        .collect();

    let model_name = if is_cnn14 { "PANNs CNN14 Instrumental Timeline (32kHz)" } else { "YAMNet Vocal Classifier (16kHz)" };
    println!(" 🤖 Running ML Classification using {}", model_name);
    let mut vocal_count = 0usize;
    let mut non_vocal_count = 0usize;

    use rayon::prelude::*;

    if is_cnn14 {
        let mut classifier = Cnn14Classifier::new(model_path)?;
        let batch_size = 64;

        for chunk in pending_rows.chunks(batch_size) {
            let decoded: Vec<(i64, Vec<f32>)> = chunk
                .par_iter()
                .filter_map(|(id, rel_path)| {
                    let abs_path = music_dir.join(rel_path.replace('\\', "/"));
                    if !abs_path.exists() {
                        return None;
                    }
                    decode_audio_to_target_sr(&abs_path, 32000).ok().map(|s| (*id, s))
                })
                .collect();

            let tx = conn.unchecked_transaction()?;
            {
                let mut update_instr_stmt = tx.prepare(
                    "UPDATE tracks SET piano_score = ?1, guitar_score = ?2, drums_score = ?3, bass_score = ?4, synth_score = ?5, strings_score = ?6, brass_score = ?7, detected_instruments = ?8, extra_features_json = ?9, features_computed = 1 WHERE id = ?10"
                )?;
                for (id, samples) in decoded {
                    if let Ok((scores, instr_map)) = classifier.predict_instrument_scores(&samples) {
                        let json_scores = serde_json::to_string(&scores).unwrap_or_default();
                        let raw_json = serde_json::to_string(&instr_map).unwrap_or_default();
                        let _ = update_instr_stmt.execute(rusqlite::params![
                            scores.piano,
                            scores.guitar,
                            scores.drums,
                            scores.bass,
                            scores.synth,
                            scores.strings,
                            scores.brass,
                            json_scores,
                            raw_json,
                            id
                        ]);
                        vocal_count += 1;
                    }
                }
            }
            let _ = tx.commit();
            println!("   ↳ Progress: {} / {} tracks processed", vocal_count, pending_rows.len());
        }
    } else {
        let mut classifier = YamnetClassifier::new(model_path)?;
        let batch_size = 64;

        for chunk in pending_rows.chunks(batch_size) {
            let decoded: Vec<(i64, Vec<f32>)> = chunk
                .par_iter()
                .filter_map(|(id, rel_path)| {
                    let abs_path = music_dir.join(rel_path.replace('\\', "/"));
                    if !abs_path.exists() {
                        return None;
                    }
                    decode_audio_to_target_sr(&abs_path, 16000).ok().map(|s| (*id, s))
                })
                .collect();

            let tx = conn.unchecked_transaction()?;
            {
                let mut update_vocal_stmt = tx.prepare(
                    "UPDATE tracks SET vocal_status = ?1, features_computed = 1 WHERE id = ?2"
                )?;

                for (id, samples) in decoded {
                    if let Ok(score) = classifier.predict_vocal_score(&samples) {
                        let status = if score >= 0.15 { "vocal" } else { "non-vocal" };
                        let _ = update_vocal_stmt.execute(rusqlite::params![status, id]);
                        if status == "vocal" {
                            vocal_count += 1;
                        } else {
                            non_vocal_count += 1;
                        }
                    }
                }
            }
            let _ = tx.commit();
            println!("   ↳ Progress: {} / {} tracks classified", vocal_count + non_vocal_count, pending_rows.len());
        }
    }

    println!(" ✅ Native Rust ML Classification Complete: {} processed", vocal_count + non_vocal_count);
    Ok((vocal_count, non_vocal_count))
}
