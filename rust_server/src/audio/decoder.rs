use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use std::path::Path;

pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_secs: f64,
}

#[allow(dead_code)]
pub fn decode_file<P: AsRef<Path>>(path: P) -> Result<DecodedAudio, String> {
    let file = File::open(path.as_ref()).map_err(|e| format!("Failed to open file: {}", e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.as_ref().extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }

    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .map_err(|e| format!("Unsupported format: {}", e))?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or("No supported audio track found")?;

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .map_err(|e| format!("Failed to create decoder: {}", e))?;

    let mut samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(ref err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(Error::ResetRequired) => break,
            Err(Error::IoError(_)) => break,
            Err(_) => break,
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                match audio_buf {
                    AudioBufferRef::F32(buf) => {
                        for frame in 0..buf.frames() {
                            for ch in 0..buf.spec().channels.count() {
                                samples.push(buf.chan(ch)[frame]);
                            }
                        }
                    }
                    AudioBufferRef::S16(buf) => {
                        for frame in 0..buf.frames() {
                            for ch in 0..buf.spec().channels.count() {
                                samples.push(buf.chan(ch)[frame] as f32 / 32768.0);
                            }
                        }
                    }
                    AudioBufferRef::S24(buf) => {
                        for frame in 0..buf.frames() {
                            for ch in 0..buf.spec().channels.count() {
                                samples.push(buf.chan(ch)[frame].inner() as f32 / 8388608.0);
                            }
                        }
                    }
                    AudioBufferRef::S32(buf) => {
                        for frame in 0..buf.frames() {
                            for ch in 0..buf.spec().channels.count() {
                                samples.push(buf.chan(ch)[frame] as f32 / 2147483648.0);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(Error::DecodeError(_)) => continue,
            Err(err) => return Err(format!("Decode failure: {}", err)),
        }
    }

    let total_frames = samples.len() / channels as usize;
    let duration_secs = total_frames as f64 / sample_rate as f64;

    Ok(DecodedAudio {
        samples,
        sample_rate,
        channels,
        duration_secs,
    })
}

#[allow(dead_code)]
pub fn resample_audio(
    input: &DecodedAudio,
    target_sample_rate: u32,
    target_channels: u16,
) -> DecodedAudio {
    if input.sample_rate == target_sample_rate && input.channels == target_channels {
        return DecodedAudio {
            samples: input.samples.clone(),
            sample_rate: input.sample_rate,
            channels: input.channels,
            duration_secs: input.duration_secs,
        };
    }

    let src_channels = input.channels as usize;
    let dst_channels = target_channels as usize;
    let src_frames = input.samples.len() / src_channels;

    if src_frames == 0 {
        return DecodedAudio {
            samples: Vec::new(),
            sample_rate: target_sample_rate,
            channels: target_channels,
            duration_secs: 0.0,
        };
    }

    // Step 1: Channel mapping
    let mut channel_mapped: Vec<f32> = Vec::with_capacity(src_frames * dst_channels);
    for frame_idx in 0..src_frames {
        let base = frame_idx * src_channels;
        if src_channels == 1 && dst_channels == 2 {
            let sample = input.samples[base];
            channel_mapped.push(sample);
            channel_mapped.push(sample);
        } else if src_channels == 2 && dst_channels == 1 {
            let left = input.samples[base];
            let right = input.samples[base + 1];
            channel_mapped.push((left + right) * 0.5);
        } else if src_channels == dst_channels {
            for ch in 0..dst_channels {
                channel_mapped.push(input.samples[base + ch]);
            }
        } else {
            for ch in 0..dst_channels {
                if ch < src_channels {
                    channel_mapped.push(input.samples[base + ch]);
                } else {
                    channel_mapped.push(0.0);
                }
            }
        }
    }

    // Step 2: Linear Interpolation Sample Rate Conversion
    if input.sample_rate == target_sample_rate {
        return DecodedAudio {
            samples: channel_mapped,
            sample_rate: target_sample_rate,
            channels: target_channels,
            duration_secs: input.duration_secs,
        };
    }

    let src_sr = input.sample_rate as f64;
    let dst_sr = target_sample_rate as f64;
    let factor = src_sr / dst_sr;

    let dst_frames = ((src_frames as f64) / factor).round() as usize;
    let mut resampled_samples = Vec::with_capacity(dst_frames * dst_channels);

    for dst_frame in 0..dst_frames {
        let src_frame_float = dst_frame as f64 * factor;
        let i0 = src_frame_float.floor() as usize;
        let i1 = (i0 + 1).min(src_frames - 1);
        let frac = (src_frame_float - i0 as f64) as f32;

        let base0 = i0 * dst_channels;
        let base1 = i1 * dst_channels;

        for ch in 0..dst_channels {
            let s0 = channel_mapped[base0 + ch];
            let s1 = channel_mapped[base1 + ch];
            let interpolated = s0 * (1.0 - frac) + s1 * frac;
            resampled_samples.push(interpolated);
        }
    }

    let duration_secs = resampled_samples.len() as f64 / (target_sample_rate as f64 * target_channels as f64);

    DecodedAudio {
        samples: resampled_samples,
        sample_rate: target_sample_rate,
        channels: target_channels,
        duration_secs,
    }
}

pub fn resample_chunk(
    input_samples: &[f32],
    src_sr: u32,
    src_channels: u16,
    dst_sr: u32,
    dst_channels: u16,
) -> Vec<f32> {
    let src_ch = src_channels as usize;
    let dst_ch = dst_channels as usize;
    let src_frames = input_samples.len() / src_ch;
    if src_frames == 0 {
        return Vec::new();
    }

    let mut ch_mapped: Vec<f32> = Vec::with_capacity(src_frames * dst_ch);
    for f in 0..src_frames {
        let base = f * src_ch;
        if src_ch == 1 && dst_ch == 2 {
            let s = input_samples[base];
            ch_mapped.push(s);
            ch_mapped.push(s);
        } else if src_ch == 2 && dst_ch == 1 {
            let l = input_samples[base];
            let r = input_samples[base + 1];
            ch_mapped.push((l + r) * 0.5);
        } else {
            for c in 0..dst_ch {
                if c < src_ch {
                    ch_mapped.push(input_samples[base + c]);
                } else {
                    ch_mapped.push(0.0);
                }
            }
        }
    }

    if src_sr == dst_sr {
        return ch_mapped;
    }

    let factor = src_sr as f64 / dst_sr as f64;
    let dst_frames = ((src_frames as f64) / factor).round() as usize;
    let mut out = Vec::with_capacity(dst_frames * dst_ch);

    for dst_f in 0..dst_frames {
        let src_f_float = dst_f as f64 * factor;
        let i0 = src_f_float.floor() as usize;
        let i1 = (i0 + 1).min(src_frames - 1);
        let frac = (src_f_float - i0 as f64) as f32;

        let b0 = i0 * dst_ch;
        let b1 = i1 * dst_ch;

        for c in 0..dst_ch {
            let s0 = ch_mapped[b0 + c];
            let s1 = ch_mapped[b1 + c];
            out.push(s0 * (1.0 - frac) + s1 * frac);
        }
    }

    out
}

pub fn extract_cover_art<P: AsRef<Path>>(path: P) -> Option<(Vec<u8>, String)> {
    let file_path = path.as_ref();
    if let Ok(file) = File::open(file_path) {
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        if let Ok(mut probed) = symphonia::default::get_probe().format(&hint, mss, &Default::default(), &Default::default()) {
            if let Some(meta) = probed.format.metadata().current() {
                for vis in meta.visuals() {
                    return Some((vis.data.to_vec(), vis.media_type.clone()));
                }
            }
            if let Some(meta_log) = probed.metadata.get() {
                if let Some(current) = meta_log.current() {
                    for vis in current.visuals() {
                        return Some((vis.data.to_vec(), vis.media_type.clone()));
                    }
                }
            }
        }
    }

    if let Some(dir) = file_path.parent() {
        for name in &["cover.jpg", "cover.png", "folder.jpg", "folder.png", "front.jpg", "front.png", "album.jpg", "album.png"] {
            let p = dir.join(name);
            if p.is_file() {
                if let Ok(bytes) = std::fs::read(&p) {
                    let mime = if name.ends_with(".png") { "image/png".to_string() } else { "image/jpeg".to_string() };
                    return Some((bytes, mime));
                }
            }
        }
    }

    None
}
