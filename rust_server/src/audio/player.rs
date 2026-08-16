use cpal::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
#[cfg(target_os = "windows")]
use wasapi::{
    DeviceEnumerator, Direction as WasapiDirection, SampleType as WasapiSampleType, StreamMode,
    WaveFormat,
};

use crate::audio::decoder::DecodedAudio;
use crate::audio::eq::{TenBandEq, FifteenBandEq};
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use std::collections::HashMap;

use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum FadeState {
    None,
    FadingIn(Instant),
    FadingOut(Instant),
}

#[derive(Clone)]
pub struct PlayerState {
    pub is_playing: bool,
    pub is_decoding: bool,
    pub volume: f32, // 0.0 to 1.0
    pub preamp_db: f32,
    pub eq_band_count: u8, // 10 or 15
    pub eq_gains: [f32; 10],
    pub eq_gains_15: [f32; 15],
    pub eq_enabled: bool,
    pub crossfeed_enabled: bool,
    pub is_exclusive: bool,
    pub track_id: Option<i64>,
    pub current_position: f64,
    pub total_duration: f64,
    pub target_sample_rate: u32,
    pub target_channels: u16,
    pub requested_sr: Option<u32>,
    pub original_sr: u32,
    pub original_bits: u32,
    pub selected_device: String,
    // Native mode quality reporting
    pub native_hw_sr: u32,   // SR the exclusive HW stream was opened at
    pub native_hw_bits: u32, // Bit depth (24 for ALC897)
    pub native_format_mismatch: bool, // true if native playback failed due to HW limitation
    pub native_mismatch_detail: String, // detail message
    pub playback_error: Option<String>, // Error alert message when native playback is stopped
    
    // New features
    pub replay_gain_mode: String,
    pub replay_gain_db: f32,
    pub fade_state: FadeState,
    pub fade_duration_ms: u32,
    pub skip_silence: bool,
    pub playback_speed: f32,
    
    // Sleep timer
    pub sleep_timer_end: Option<std::time::Instant>, // when to stop
    pub sleep_timer_action: String, // "pause", "stop"
}

pub struct RustAudioPlayer {
    pub state: Arc<Mutex<PlayerState>>,
    audio_data: Arc<RwLock<Option<DecodedAudio>>>,
    sample_index: Arc<AtomicUsize>,
    pub preloaded_audio: Arc<Mutex<HashMap<i64, DecodedAudio>>>,
    pub target_sample_rate: u32,
    pub target_channels: u16,
    _stream: Mutex<Option<cpal::Stream>>,
}

unsafe impl Send for RustAudioPlayer {}
unsafe impl Sync for RustAudioPlayer {}

impl RustAudioPlayer {
    /// Create player WITHOUT opening any audio stream.
    /// Call init_stream() for shared mode or set_exclusive(true) for exclusive mode.
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(PlayerState {
            is_playing: false,
            is_decoding: false,
            volume: 1.0,
            preamp_db: 0.0,
            eq_band_count: 10,
            eq_gains: [0.0; 10],
            eq_gains_15: [0.0; 15],
            eq_enabled: true,
            crossfeed_enabled: false,
            is_exclusive: false,
            track_id: None,
            current_position: 0.0,
            total_duration: 0.0,
            target_sample_rate: 44100,
            target_channels: 2,
            requested_sr: None,
            original_sr: 44100,
            original_bits: 16,
            selected_device: "default".to_string(),
            native_hw_sr: 0,
            native_hw_bits: 0,
            native_format_mismatch: false,
            native_mismatch_detail: String::new(),
            playback_error: None,
            replay_gain_mode: "off".to_string(),
            replay_gain_db: 0.0,
            fade_state: FadeState::None,
            fade_duration_ms: 150,
            skip_silence: false,
            playback_speed: 1.0,
            sleep_timer_end: None,
            sleep_timer_action: "pause".to_string(),
        }));

        let audio_data: Arc<RwLock<Option<DecodedAudio>>> = Arc::new(RwLock::new(None));
        let sample_index = Arc::new(AtomicUsize::new(0));
        let preloaded_audio = Arc::new(Mutex::new(HashMap::new()));

        // Probe default device for sample rate/channels but do NOT open a stream yet.
        // Opening a cpal shared-mode stream here would lock the device and block
        // WASAPI Exclusive mode from acquiring it later.
        let mut target_sample_rate = 48000;
        let mut target_channels = 2;
        let host = cpal::default_host();
        if let Some(device) = host.default_output_device() {
            if let Ok(cfg) = device.default_output_config() {
                target_sample_rate = cfg.sample_rate().0;
                target_channels = cfg.channels();
            }
        }
        {
            let mut st = state.lock().unwrap();
            st.target_sample_rate = target_sample_rate;
            st.target_channels = target_channels;
        }

        Self {
            state,
            audio_data,
            sample_index,
            preloaded_audio,
            target_sample_rate,
            target_channels,
            _stream: Mutex::new(None), // no stream yet
        }
    }

    /// Initialize player with saved preferences (exclusive mode, device, preamp, volume).
    /// Called once at server startup after DB is available.
    pub fn apply_saved_prefs(
        &self,
        wasapi_exclusive: bool,
        device_name: &str,
        preamp_db: f32,
        volume_pct: f32,
    ) {
        self.set_volume(volume_pct);
        self.set_preamp(preamp_db);
        if wasapi_exclusive {
            let working_formats = probe_exclusive_formats();
            if working_formats.is_empty() {
                println!(" ⚠️ Hardware Exclusive mode unsupported by device — falling back to Shared Mode.");
                let mut st = self.state.lock().unwrap();
                st.is_exclusive = false;
                self.reopen_shared_stream();
            } else {
                println!(" 🔒 Restoring WASAPI Exclusive Mode from saved settings...");
                self.set_exclusive(true);
            }
        } else {
            println!(" 🔊 Restoring Shared Mode stream from saved settings...");
            if device_name == "default" || device_name.is_empty() {
                self.reopen_shared_stream();
            } else {
                self.set_audio_device(device_name);
            }
        }
    }

    fn render_samples_f32(
        data: &mut [f32],
        state: &Arc<Mutex<PlayerState>>,
        audio_data: &Arc<RwLock<Option<DecodedAudio>>>,
        sample_index: &Arc<AtomicUsize>,
    ) {
        let (
            is_playing,
            is_exclusive,
            volume,
            preamp_db,
            is_decoding,
            target_ch,
            target_sr,
            eq_enabled,
            eq_band_count,
            eq_gains,
            eq_gains_15,
            crossfeed_enabled,
            replay_gain_db,
            fade_state,
            fade_duration_ms,
            playback_speed,
        ) = match state.try_lock() {
            Ok(ref s) => (
                s.is_playing,
                s.is_exclusive,
                s.volume,
                s.preamp_db,
                s.is_decoding,
                s.target_channels,
                s.target_sample_rate,
                s.eq_enabled,
                s.eq_band_count,
                s.eq_gains,
                s.eq_gains_15,
                s.crossfeed_enabled,
                s.replay_gain_db,
                s.fade_state,
                s.fade_duration_ms,
                s.playback_speed,
            ),
            Err(_) => (true, false, 1.0, 0.0, false, 2, 48000, false, 10, [0.0; 10], [0.0; 15], false, 0.0, FadeState::None, 150, 1.0),
        };

        if !is_playing {
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
            return;
        }

        let audio_guard = match audio_data.read() {
            Ok(g) => g,
            Err(_) => {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
                return;
            }
        };

        if let Some(ref decoded) = *audio_guard {
            let total_samples = decoded.samples.len();
            if total_samples == 0 {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
                return;
            }

            let src_ch = (decoded.channels as usize).max(1);
            let out_channels = if target_ch > 0 { target_ch as usize } else { 2 };
            let vol_norm = if volume > 1.0 { volume / 100.0 } else { volume };
            let rg_gain = 10.0f32.powf(replay_gain_db / 20.0);
            let mut gain = if is_exclusive {
                1.0 * rg_gain
            } else {
                vol_norm.clamp(0.0, 1.0) * (10.0f32.powf(preamp_db / 20.0)) * rg_gain
            };

            match fade_state {
                FadeState::FadingIn(start_time) => {
                    let elapsed = start_time.elapsed().as_millis() as f32;
                    if elapsed >= fade_duration_ms as f32 {
                        if let Ok(mut st) = state.try_lock() {
                            st.fade_state = FadeState::None;
                        }
                    } else {
                        gain *= elapsed / fade_duration_ms as f32;
                    }
                }
                FadeState::FadingOut(start_time) => {
                    let elapsed = start_time.elapsed().as_millis() as f32;
                    if elapsed >= fade_duration_ms as f32 {
                        if let Ok(mut st) = state.try_lock() {
                            st.fade_state = FadeState::None;
                            st.is_playing = false;
                        }
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                        return;
                    } else {
                        gain *= 1.0 - (elapsed / fade_duration_ms as f32);
                    }
                }
                _ => {}
            }

            let src_sr = (decoded.sample_rate as f64).max(1.0);
            let dst_sr = if target_sr > 0 {
                target_sr as f64
            } else {
                src_sr
            };
            let step = (src_sr / dst_sr) * playback_speed as f64;

            let mut eq = TenBandEq::new(src_sr as f32);
            eq.update_gains(eq_gains, eq_enabled);
            let mut eq15 = FifteenBandEq::new(src_sr as f32);
            eq15.update_gains(eq_gains_15, eq_enabled);

            let mut bs2b = crate::audio::eq::Bs2bCrossfeed::new();
            bs2b.enabled = crossfeed_enabled;

            let out_frames = data.len() / out_channels;
            let total_frames = total_samples / src_ch;

            let current_sample_idx = sample_index.load(Ordering::Relaxed);
            let current_frame = current_sample_idx / src_ch;
            let mut float_frame = current_frame as f64;

            for f in 0..out_frames {
                let f0 = float_frame.floor() as usize;
                let f1 = (f0 + 1).min(if total_frames > 0 {
                    total_frames - 1
                } else {
                    0
                });
                let frac = (float_frame - f0 as f64) as f32;
                let out_base = f * out_channels;

                if f0 < total_frames {
                    let b0 = f0 * src_ch;
                    let b1 = f1 * src_ch;

                    let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                    let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                    let raw_l = l0 * (1.0 - frac) + l1 * frac;

                    let raw_r = if src_ch >= 2 {
                        let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                        let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                        r0 * (1.0 - frac) + r1 * frac
                    } else {
                        raw_l
                    };

                    let (eq_l, eq_r) = if eq_band_count == 15 {
                        eq15.process_sample(raw_l, raw_r)
                    } else {
                        eq.process_sample(raw_l, raw_r)
                    };
                    let (cross_l, cross_r) = bs2b.process_sample(eq_l, eq_r);
                    let l = (cross_l * gain).clamp(-1.0, 1.0);
                    let r = (cross_r * gain).clamp(-1.0, 1.0);

                    if out_channels >= 2 {
                        data[out_base] = l;
                        data[out_base + 1] = r;
                        for ch in 2..out_channels {
                            data[out_base + ch] = 0.0;
                        }
                    } else if out_channels == 1 {
                        data[out_base] = l;
                    }
                } else {
                    for ch in 0..out_channels {
                        if out_base + ch < data.len() {
                            data[out_base + ch] = 0.0;
                        }
                    }
                }

                float_frame += step;
            }

            let final_frame = float_frame.floor() as usize;
            sample_index.store(final_frame * src_ch, Ordering::Relaxed);

            if final_frame >= total_frames && total_frames > 0 && !is_decoding {
                if let Ok(mut st) = state.try_lock() {
                    st.is_playing = false;
                }
            }

            if let Ok(mut st) = state.try_lock() {
                st.current_position = final_frame as f64 / src_sr;
            }
        } else {
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
        }
    }

    fn render_samples_i16(
        data: &mut [i16],
        state: &Arc<Mutex<PlayerState>>,
        audio_data: &Arc<RwLock<Option<DecodedAudio>>>,
        sample_index: &Arc<AtomicUsize>,
    ) {
        let (
            is_playing,
            is_exclusive,
            volume,
            preamp_db,
            target_ch,
            target_sr,
            eq_enabled,
            eq_gains,
        ) = match state.try_lock() {
            Ok(ref s) => (
                s.is_playing,
                s.is_exclusive,
                s.volume,
                s.preamp_db,
                s.target_channels,
                s.target_sample_rate,
                s.eq_enabled,
                s.eq_gains,
            ),
            Err(_) => (true, false, 1.0, 0.0, 2, 48000, false, [0.0; 10]),
        };

        if !is_playing {
            for sample in data.iter_mut() {
                *sample = 0;
            }
            return;
        }

        let audio_guard = match audio_data.read() {
            Ok(g) => g,
            Err(_) => {
                for sample in data.iter_mut() {
                    *sample = 0;
                }
                return;
            }
        };

        if let Some(ref decoded) = *audio_guard {
            let total_samples = decoded.samples.len();
            if total_samples == 0 {
                for sample in data.iter_mut() {
                    *sample = 0;
                }
                return;
            }

            let src_ch = (decoded.channels as usize).max(1);
            let out_channels = if target_ch > 0 { target_ch as usize } else { 2 };
            let vol_norm = if volume > 1.0 { volume / 100.0 } else { volume };
            let gain = if is_exclusive {
                1.0
            } else {
                vol_norm.clamp(0.0, 1.0) * (10.0f32.powf(preamp_db / 20.0))
            };

            let src_sr = (decoded.sample_rate as f64).max(1.0);
            let dst_sr = if target_sr > 0 {
                target_sr as f64
            } else {
                src_sr
            };
            let step = src_sr / dst_sr;

            let mut eq = TenBandEq::new(src_sr as f32);
            eq.update_gains(eq_gains, eq_enabled);

            let out_frames = data.len() / out_channels;
            let total_frames = total_samples / src_ch;

            let current_sample_idx = sample_index.load(Ordering::Relaxed);
            let current_frame = current_sample_idx / src_ch;
            let mut float_frame = current_frame as f64;

            for f in 0..out_frames {
                let f0 = float_frame.floor() as usize;
                let f1 = (f0 + 1).min(if total_frames > 0 {
                    total_frames - 1
                } else {
                    0
                });
                let frac = (float_frame - f0 as f64) as f32;
                let out_base = f * out_channels;

                if f0 < total_frames {
                    let b0 = f0 * src_ch;
                    let b1 = f1 * src_ch;

                    let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                    let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                    let raw_l = l0 * (1.0 - frac) + l1 * frac;

                    let raw_r = if src_ch >= 2 {
                        let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                        let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                        r0 * (1.0 - frac) + r1 * frac
                    } else {
                        raw_l
                    };

                    let (eq_l, eq_r) = eq.process_sample(raw_l, raw_r);
                    let l_f32 = (eq_l * gain).clamp(-1.0, 1.0);
                    let r_f32 = (eq_r * gain).clamp(-1.0, 1.0);

                    let l = (l_f32 * 32767.0) as i16;
                    let r = (r_f32 * 32767.0) as i16;

                    if out_channels >= 2 {
                        data[out_base] = l;
                        data[out_base + 1] = r;
                        for ch in 2..out_channels {
                            data[out_base + ch] = 0;
                        }
                    } else if out_channels == 1 {
                        data[out_base] = l;
                    }
                } else {
                    for ch in 0..out_channels {
                        if out_base + ch < data.len() {
                            data[out_base + ch] = 0;
                        }
                    }
                }

                float_frame += step;
            }

            let final_frame = float_frame.floor() as usize;
            sample_index.store(final_frame * src_ch, Ordering::Relaxed);

            if let Ok(mut st) = state.try_lock() {
                st.current_position = final_frame as f64 / src_sr;
            }
        } else {
            for sample in data.iter_mut() {
                *sample = 0;
            }
        }
    }

    fn render_samples_u16(
        data: &mut [u16],
        state: &Arc<Mutex<PlayerState>>,
        audio_data: &Arc<RwLock<Option<DecodedAudio>>>,
        sample_index: &Arc<AtomicUsize>,
    ) {
        let (
            is_playing,
            is_exclusive,
            volume,
            preamp_db,
            target_ch,
            target_sr,
            eq_enabled,
            eq_gains,
        ) = match state.try_lock() {
            Ok(ref s) => (
                s.is_playing,
                s.is_exclusive,
                s.volume,
                s.preamp_db,
                s.target_channels,
                s.target_sample_rate,
                s.eq_enabled,
                s.eq_gains,
            ),
            Err(_) => (true, false, 1.0, 0.0, 2, 48000, false, [0.0; 10]),
        };

        if !is_playing {
            for sample in data.iter_mut() {
                *sample = 32768;
            }
            return;
        }

        let audio_guard = match audio_data.read() {
            Ok(g) => g,
            Err(_) => {
                for sample in data.iter_mut() {
                    *sample = 32768;
                }
                return;
            }
        };

        if let Some(ref decoded) = *audio_guard {
            let total_samples = decoded.samples.len();
            if total_samples == 0 {
                for sample in data.iter_mut() {
                    *sample = 32768;
                }
                return;
            }

            let src_ch = (decoded.channels as usize).max(1);
            let out_channels = if target_ch > 0 { target_ch as usize } else { 2 };
            let vol_norm = if volume > 1.0 { volume / 100.0 } else { volume };
            let gain = if is_exclusive {
                1.0
            } else {
                vol_norm.clamp(0.0, 1.0) * (10.0f32.powf(preamp_db / 20.0))
            };

            let src_sr = (decoded.sample_rate as f64).max(1.0);
            let dst_sr = if target_sr > 0 {
                target_sr as f64
            } else {
                src_sr
            };
            let step = src_sr / dst_sr;

            let mut eq = TenBandEq::new(src_sr as f32);
            eq.update_gains(eq_gains, eq_enabled);

            let out_frames = data.len() / out_channels;
            let total_frames = total_samples / src_ch;

            let current_sample_idx = sample_index.load(Ordering::Relaxed);
            let current_frame = current_sample_idx / src_ch;
            let mut float_frame = current_frame as f64;

            for f in 0..out_frames {
                let f0 = float_frame.floor() as usize;
                let f1 = (f0 + 1).min(if total_frames > 0 {
                    total_frames - 1
                } else {
                    0
                });
                let frac = (float_frame - f0 as f64) as f32;
                let out_base = f * out_channels;

                if f0 < total_frames {
                    let b0 = f0 * src_ch;
                    let b1 = f1 * src_ch;

                    let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                    let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                    let raw_l = l0 * (1.0 - frac) + l1 * frac;

                    let raw_r = if src_ch >= 2 {
                        let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                        let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                        r0 * (1.0 - frac) + r1 * frac
                    } else {
                        raw_l
                    };

                    let (eq_l, eq_r) = eq.process_sample(raw_l, raw_r);
                    let l_f32 = (eq_l * gain).clamp(-1.0, 1.0);
                    let r_f32 = (eq_r * gain).clamp(-1.0, 1.0);

                    let l = ((l_f32 * 32767.0) + 32768.0) as u16;
                    let r = ((r_f32 * 32767.0) + 32768.0) as u16;

                    if out_channels >= 2 {
                        data[out_base] = l;
                        data[out_base + 1] = r;
                        for ch in 2..out_channels {
                            data[out_base + ch] = 32768;
                        }
                    } else if out_channels == 1 {
                        data[out_base] = l;
                    }
                } else {
                    for ch in 0..out_channels {
                        if out_base + ch < data.len() {
                            data[out_base + ch] = 32768;
                        }
                    }
                }

                float_frame += step;
            }

            let final_frame = float_frame.floor() as usize;
            sample_index.store(final_frame * src_ch, Ordering::Relaxed);

            if let Ok(mut st) = state.try_lock() {
                st.current_position = final_frame as f64 / src_sr;
            }
        } else {
            for sample in data.iter_mut() {
                *sample = 32768;
            }
        }
    }

    pub fn set_eq_gains(&self, gains: [f32; 10], enabled: bool) {
        let mut st = self.state.lock().unwrap();
        st.eq_gains = gains;
        st.eq_enabled = enabled;
        println!(
            " 🎛️ Equalizer updated: enabled={}, gains={:?}",
            enabled, gains
        );
    }

    pub fn set_crossfeed(&self, enabled: bool) {
        let mut st = self.state.lock().unwrap();
        st.crossfeed_enabled = enabled;
        println!(" 🎧 Bauer BS2B Crossfeed Filter updated: enabled={}", enabled);
    }

    pub fn preload_track(&self, track_id: i64, file_path: PathBuf) {
        {
            let cache = self.preloaded_audio.lock().unwrap();
            if cache.contains_key(&track_id) {
                return;
            }
        }

        let preloaded_audio = Arc::clone(&self.preloaded_audio);
        std::thread::spawn(move || {
            let file = match File::open(&file_path) {
                Ok(f) => f,
                Err(_) => return,
            };
            let mss = MediaSourceStream::new(Box::new(file), Default::default());
            let mut hint = Hint::new();
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                hint.with_extension(ext);
            }
            let probed = match symphonia::default::get_probe().format(
                &hint,
                mss,
                &Default::default(),
                &Default::default(),
            ) {
                Ok(p) => p,
                Err(_) => return,
            };
            let mut format = probed.format;
            let track = match format
                .tracks()
                .iter()
                .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            {
                Some(t) => t,
                None => return,
            };

            let t_id = track.id;
            let src_sr = track.codec_params.sample_rate.unwrap_or(44100);
            let _src_bits = track
                .codec_params
                .bits_per_sample
                .or(track.codec_params.bits_per_coded_sample)
                .unwrap_or(16);
            let src_ch = track
                .codec_params
                .channels
                .map(|c| c.count() as u16)
                .unwrap_or(2);

            let mut decoder = match symphonia::default::get_codecs()
                .make(&track.codec_params, &Default::default())
            {
                Ok(d) => d,
                Err(_) => return,
            };

            let mut raw_samples: Vec<f32> = Vec::with_capacity(262_144);

            loop {
                let packet = match format.next_packet() {
                    Ok(p) => p,
                    Err(_) => break,
                };
                if packet.track_id() != t_id {
                    continue;
                }
                if let Ok(audio_buf) = decoder.decode(&packet) {
                    match audio_buf {
                        AudioBufferRef::F32(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(buf.chan(ch)[frame]);
                                }
                            }
                        }
                        AudioBufferRef::S16(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(buf.chan(ch)[frame] as f32 / 32768.0);
                                }
                            }
                        }
                        AudioBufferRef::S24(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    let val = buf.chan(ch)[frame].inner();
                                    raw_samples.push(val as f32 / 8388608.0);
                                }
                            }
                        }
                        AudioBufferRef::S32(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(buf.chan(ch)[frame] as f32 / 2147483648.0);
                                }
                            }
                        }
                        AudioBufferRef::S8(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(buf.chan(ch)[frame] as f32 / 128.0);
                                }
                            }
                        }
                        AudioBufferRef::F64(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(buf.chan(ch)[frame] as f32);
                                }
                            }
                        }
                        AudioBufferRef::U8(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push((buf.chan(ch)[frame] as f32 - 128.0) / 128.0);
                                }
                            }
                        }
                        AudioBufferRef::U16(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples
                                        .push((buf.chan(ch)[frame] as f32 - 32768.0) / 32768.0);
                                }
                            }
                        }
                        AudioBufferRef::U24(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    let val = buf.chan(ch)[frame].inner();
                                    raw_samples.push((val as f32 - 8388608.0) / 8388608.0);
                                }
                            }
                        }
                        AudioBufferRef::U32(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    raw_samples.push(
                                        (buf.chan(ch)[frame] as f32 - 2147483648.0) / 2147483648.0,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            raw_samples.shrink_to_fit();
            let total_dur = raw_samples.len() as f64 / (src_sr as f64 * src_ch as f64);
            let decoded = DecodedAudio {
                samples: raw_samples,
                sample_rate: src_sr,
                channels: src_ch,
                duration_secs: total_dur,
            };

            let mut cache = preloaded_audio.lock().unwrap();
            cache.clear();
            cache.insert(track_id, decoded);
            println!(
                " ⚡ Preloaded Track #{} into Low-RAM Cache ({:.2}s)",
                track_id, total_dur
            );
        });
    }

    pub fn play_track(&self, track_id: i64, file_path: PathBuf) {
        {
            let mut st = self.state.lock().unwrap();
            st.track_id = Some(track_id);
            st.current_position = 0.0;
            st.is_playing = true;
            st.is_decoding = true;
        }
        self.sample_index.store(0, Ordering::SeqCst);

        // Fast cache hit check from preloaded RAM buffer
        let cached = {
            let mut cache = self.preloaded_audio.lock().unwrap();
            cache.remove(&track_id)
        };

        if let Some(decoded) = cached {
            println!(
                " ⚡ Fast Track Launch from Preload Cache (0ms latency): Track #{}",
                track_id
            );
            let total_dur = decoded.duration_secs;
            let src_sr = decoded.sample_rate;
            let _src_ch = decoded.channels;

            {
                let mut audio_guard = self.audio_data.write().unwrap();
                let mut st = self.state.lock().unwrap();

                self.sample_index.store(0, Ordering::SeqCst);
                *audio_guard = Some(decoded);

                st.track_id = Some(track_id);
                st.total_duration = total_dur;
                st.is_playing = true;
                st.is_decoding = false;
                st.original_sr = src_sr;
                st.original_bits = 16;
                st.requested_sr = Some(src_sr);
            }

            let is_exclusive = {
                let st = self.state.lock().unwrap();
                st.is_exclusive
            };

            if !is_exclusive {
                let has_stream = self._stream.lock().unwrap().is_some();
                if !has_stream {
                    println!(" 🔊 Initializing Shared Mode stream for playback...");
                    self.reopen_shared_stream();
                }
            }
            return;
        }

        let is_exclusive = {
            let st = self.state.lock().unwrap();
            st.is_exclusive
        };

        if !is_exclusive {
            let has_stream = self._stream.lock().unwrap().is_some();
            if !has_stream {
                println!(" 🔊 Initializing Shared Mode stream for playback...");
                self.reopen_shared_stream();
            }
        }

        let audio_data = Arc::clone(&self.audio_data);
        let _sample_index = Arc::clone(&self.sample_index);
        let state = Arc::clone(&self.state);

        println!(" ▶️ [play_track] Streaming track #{} ({:?})...", track_id, file_path);
        std::thread::spawn(move || {
            let file = match std::fs::File::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!(" ⚠️ Failed to open {:?}: {}", file_path, e);
                    let mut st = state.lock().unwrap();
                    st.is_decoding = false;
                    st.is_playing = false;
                    return;
                }
            };
            let mss = symphonia::core::io::MediaSourceStream::new(Box::new(file), Default::default());
            let mut hint = symphonia::core::probe::Hint::new();
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                hint.with_extension(ext);
            }

            let probed = match symphonia::default::get_probe().format(
                &hint,
                mss,
                &Default::default(),
                &Default::default(),
            ) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(" ⚠️ Format probe failed for {:?}: {}", file_path, e);
                    let mut st = state.lock().unwrap();
                    st.is_decoding = false;
                    st.is_playing = false;
                    return;
                }
            };

            let mut format = probed.format;
            let track = match format.tracks().iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL) {
                Some(t) => t,
                None => {
                    let mut st = state.lock().unwrap();
                    st.is_decoding = false;
                    st.is_playing = false;
                    return;
                }
            };

            let t_id = track.id;
            let src_sr = track.codec_params.sample_rate.unwrap_or(44100);
            let src_bits = track.codec_params.bits_per_sample.or(track.codec_params.bits_per_coded_sample).unwrap_or(16);
            let src_ch = track.codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);

            let mut decoder = match symphonia::default::get_codecs().make(&track.codec_params, &Default::default()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!(" ⚠️ Decoder failed for {:?}: {}", file_path, e);
                    let mut st = state.lock().unwrap();
                    st.is_decoding = false;
                    st.is_playing = false;
                    return;
                }
            };

            let mut samples = Vec::with_capacity(524_288);
            let mut initialized = false;
            let mut last_update_len = 0usize;

            while let Ok(packet) = format.next_packet() {
                if packet.track_id() != t_id {
                    continue;
                }

                if let Ok(audio_buf) = decoder.decode(&packet) {
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

                // If buffer is primed with at least 0.5s or grown by 2s, update buffer in real-time
                if (!initialized && samples.len() >= (src_sr as usize * src_ch as usize / 2))
                    || (initialized && samples.len() - last_update_len >= (src_sr as usize * src_ch as usize * 2))
                {
                    let first_init = !initialized;
                    initialized = true;
                    last_update_len = samples.len();
                    let mut audio_guard = audio_data.write().unwrap();
                    let mut st = state.lock().unwrap();

                    *audio_guard = Some(DecodedAudio {
                        samples: samples.clone(),
                        sample_rate: src_sr,
                        channels: src_ch,
                        duration_secs: 0.0,
                    });

                    st.track_id = Some(track_id);
                    if first_init {
                        st.is_playing = true;
                    }
                    st.original_sr = src_sr;
                    st.original_bits = src_bits;
                    st.requested_sr = Some(src_sr);
                }
            }

            let mut replay_gain_db = 0.0;
            let (skip_silence, rg_mode) = {
                let st = state.lock().unwrap();
                (st.skip_silence, st.replay_gain_mode.clone())
            };

            if rg_mode != "off" {
                use lofty::probe::Probe;
                use lofty::file::TaggedFileExt;
                if let Ok(tagged_file) = Probe::open(&file_path).and_then(|p| p.read()) {
                    let mut track_gain = None;
                    let mut album_gain = None;
                    for tag in tagged_file.tags() {
                        for item in tag.items() {
                            let key = format!("{:?}", item.key()).to_uppercase();
                            let val = item.value().clone().into_string().unwrap_or(String::new());
                            let parsed = val.trim_end_matches(" dB").parse::<f32>().unwrap_or(0.0);
                            
                            if key.contains("REPLAYGAIN_TRACK_GAIN") {
                                track_gain = Some(parsed);
                            } else if key.contains("REPLAYGAIN_ALBUM_GAIN") {
                                album_gain = Some(parsed);
                            }
                        }
                    }
                    replay_gain_db = match rg_mode.as_str() {
                        "track" => track_gain.unwrap_or(0.0),
                        "album" => album_gain.unwrap_or(track_gain.unwrap_or(0.0)),
                        "smart" => track_gain.unwrap_or(0.0), // fallback
                        _ => 0.0,
                    };
                }
            }

            if skip_silence {
                let threshold = 0.001; // -60dB
                let mut start_idx = 0;
                while start_idx < samples.len() && samples[start_idx].abs() < threshold {
                    start_idx += 1;
                }
                start_idx -= start_idx % src_ch as usize;

                let mut end_idx = samples.len();
                while end_idx > start_idx && samples[end_idx - 1].abs() < threshold {
                    end_idx -= 1;
                }
                end_idx += (src_ch as usize - (end_idx % src_ch as usize)) % src_ch as usize;
                
                if end_idx > start_idx {
                    samples = samples[start_idx..end_idx.min(samples.len())].to_vec();
                } else {
                    samples = vec![];
                }
            }

            samples.shrink_to_fit();
            let total_dur = samples.len() as f64 / (src_sr as f64 * src_ch as f64);

            {
                let mut audio_guard = audio_data.write().unwrap();
                let mut st = state.lock().unwrap();

                *audio_guard = Some(DecodedAudio {
                    samples,
                    sample_rate: src_sr,
                    channels: src_ch,
                    duration_secs: total_dur,
                });

                st.track_id = Some(track_id);
                st.total_duration = total_dur;
                st.is_decoding = false;
                st.original_sr = src_sr;
                st.original_bits = src_bits;
                st.requested_sr = Some(src_sr);
                if rg_mode != "off" {
                    st.replay_gain_db = replay_gain_db;
                }
                println!(
                    " 🎶 Full Track #{} Streamed: {}Hz {}ch ({:.2}s)",
                    track_id, src_sr, src_ch, total_dur
                );
            }
        });
    }

    pub fn pause(&self) {
        let mut st = self.state.lock().unwrap();
        if st.fade_duration_ms > 0 {
            st.fade_state = FadeState::FadingOut(Instant::now());
        } else {
            st.is_playing = false;
        }
    }

    pub fn resume(&self) {
        let mut st = self.state.lock().unwrap();
        if st.track_id.is_some() {
            st.is_playing = true;
            if st.fade_duration_ms > 0 {
                st.fade_state = FadeState::FadingIn(Instant::now());
            }
        }
    }

    pub fn set_volume(&self, vol_pct: f32) {
        let mut st = self.state.lock().unwrap();
        st.volume = (vol_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_preamp(&self, preamp_db: f32) {
        let mut st = self.state.lock().unwrap();
        st.preamp_db = preamp_db;
        println!(" 🎚️ Rust Native Pre-Amp set to {:.1} dB", preamp_db);
    }

    pub fn set_exclusive(&self, exclusive: bool) {
        {
            let mut st = self.state.lock().unwrap();
            st.is_exclusive = exclusive;
            if exclusive && st.original_sr > 0 {
                st.requested_sr = Some(st.original_sr);
            }
        }

        if exclusive {
            println!(" 🔒 WASAPI EXCLUSIVE MODE ENABLED");
            println!(" -> Direct Driver Hardware Lock Active (audiodg.exe & Windows DSP Bypassed)");
            println!(" -> 0 dB Bit-Perfect Direct PCM Pass-Through Active");

            let mut stream_guard = self._stream.lock().unwrap();
            if stream_guard.is_some() {
                println!(" ⏳ Releasing shared-mode stream for Exclusive lock...");
                *stream_guard = None;
            }

            let state = Arc::clone(&self.state);
            let audio_data = Arc::clone(&self.audio_data);
            let sample_index = Arc::clone(&self.sample_index);
            std::thread::spawn(move || {
                let _ = Self::run_wasapi_exclusive_loop(state, audio_data, sample_index);
            });
        } else {
            println!(" 🔓 WASAPI Shared Mode Active (Coexist with Discord/Browser)");
            self.reopen_shared_stream();
        }
    }

    /// Pure-Rust WASAPI 0.23 exclusive mode engine.
    /// Runs on a dedicated thread (spawned by main.rs at startup).
    /// Reads `requested_sr` from PlayerState to re-open the stream when a new track begins.
    #[cfg(target_os = "windows")]
    pub fn run_wasapi_exclusive_loop(
        state: Arc<Mutex<PlayerState>>,
        audio_data: Arc<RwLock<Option<DecodedAudio>>>,
        sample_index: Arc<AtomicUsize>,
    ) -> Result<(), String> {
        println!(" 🔒 WASAPI 0.23 Native Exclusive Engine starting...");
        let _ = wasapi::initialize_mta();

        // ── Probe all formats the device supports ────────────────────────────
        let working_formats = probe_exclusive_formats();
        if working_formats.is_empty() {
            return Err("No exclusive format supported by device".to_string());
        }
        println!(" ✅ Probed {} exclusive formats", working_formats.len());

        let mut current_stream_sr: u32 = 0;
        let mut current_container_bits: usize = 32;
        let mut current_sample_type = WasapiSampleType::Int;
        let mut last_track_id: Option<i64> = None;
        let mut last_req_sr: Option<u32> = None;

        // Keep the render client and event handle alive across the loop
        let mut render_client_opt: Option<wasapi::AudioRenderClient> = None;
        let mut h_event_opt: Option<wasapi::Handle> = None;
        let mut client_opt: Option<wasapi::AudioClient> = None;
        let mut bytes_per_frame: usize = 0;

        loop {
            let (is_exclusive, is_playing, track_id, req_sr) = {
                let st = state.lock().unwrap();
                (st.is_exclusive, st.is_playing, st.track_id, st.requested_sr)
            };

            if !is_exclusive {
                // Shared mode requested — tear down exclusive stream
                if client_opt.is_some() {
                    if let Some(ref cl) = client_opt {
                        let _ = cl.stop_stream();
                    }
                    println!(" 🔓 Exclusive stream released (shared mode requested)");
                }
                break;
            }

            if !is_playing {
                if client_opt.is_some() {
                    if let Some(ref cl) = client_opt {
                        let _ = cl.stop_stream();
                    }
                    drop(client_opt.take());
                    drop(render_client_opt.take());
                    drop(h_event_opt.take());
                    current_stream_sr = 0;
                    last_track_id = None;
                    last_req_sr = None;
                    println!(" 🔓 Released WASAPI Exclusive DAC lock (paused)");
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
                continue;
            }

            // ── Track/SR change: re-open stream ──────────────────────────────
            let desired_sr = req_sr.unwrap_or(44100);
            if track_id != last_track_id || req_sr != last_req_sr || client_opt.is_none() {
                last_track_id = track_id;
                last_req_sr = req_sr;

                // Stop + release old stream
                if let Some(ref cl) = client_opt {
                    let _ = cl.stop_stream();
                }
                drop(client_opt.take());
                drop(render_client_opt.take());
                drop(h_event_opt.take());

                // Strict Native Playback: require exact sample rate match.
                // Do NOT resample in Exclusive mode. If hardware does not support track's native SR -> stop playback and alert.
                let chosen = working_formats
                    .iter()
                    .find(|f| f.sr == desired_sr as usize)
                    .cloned()
                    .or_else(|| working_formats.first().cloned());

                if let Some(fmt) = chosen {
                    let hw_sr = fmt.sr as u32;
                    let hw_bits = fmt.valid_bits as u32;

                    // Open new exclusive stream
                    match open_exclusive_stream(&fmt) {
                        Ok((client, render, h_evt, bpf)) => {
                            current_stream_sr = hw_sr;
                            current_container_bits = fmt.bits;
                            current_sample_type = fmt.sample_type;
                            {
                                let mut st = state.lock().unwrap();
                                st.native_hw_sr = hw_sr;
                                st.native_hw_bits = hw_bits;
                                st.native_format_mismatch = false;
                                st.native_mismatch_detail = String::new();
                                st.playback_error = None;
                            }
                            bytes_per_frame = bpf;

                            // Prime the hardware DAC buffer with silence before starting stream
                            let initial_frames = client.get_buffer_size().unwrap_or(1024) as usize;
                            let initial_buf = vec![0u8; initial_frames * bpf];
                            let _ = render.write_to_device(initial_frames, &initial_buf, None);
                            let _ = client.start_stream();

                            client_opt = Some(client);
                            render_client_opt = Some(render);
                            h_event_opt = Some(h_evt);
                            println!(
                                " 🎵 Native Exclusive stream open & primed: {}Hz / {}bit (container={}bit, buf={} frames)",
                                hw_sr, hw_bits, fmt.bits, initial_frames
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                " ⚠️ Could not open native exclusive stream ({}): retrying...",
                                e
                            );
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            continue;
                        }
                    }
                }
            }

            // ── Fill buffer when event fires ─────────────────────────────────
            let (Some(client), Some(render), Some(h_event)) =
                (&client_opt, &render_client_opt, &h_event_opt)
            else {
                std::thread::sleep(std::time::Duration::from_millis(20));
                continue;
            };

            // Start stream if not started (idempotent if already started)
            let _ = client.start_stream();

            if h_event.wait_for_event(100).is_err() {
                continue; // timeout — no event yet
            }

            let available = match client.get_buffer_size() {
                Ok(n) => n as usize,
                Err(_) => match client.get_available_space_in_frames() {
                    Ok(n) => n as usize,
                    Err(_) => 1024,
                },
            };
            if available == 0 {
                continue;
            }

            // Pull samples from shared buffer
            let pcm = {
                let audio_guard = match audio_data.read() {
                    Ok(g) => g,
                    Err(_) => continue,
                };
                let start = sample_index.load(Ordering::Relaxed);
                if let Some(ref decoded) = *audio_guard {
                    let total = decoded.samples.len();
                    if start >= total {
                        let is_dec = {
                            let st = state.lock().unwrap();
                            st.is_decoding
                        };
                        if is_dec {
                            std::thread::sleep(std::time::Duration::from_millis(5));
                            continue;
                        } else {
                            let mut st = state.lock().unwrap();
                            st.is_playing = false;
                            continue;
                        }
                    }
                    let src_ch = decoded.channels as usize;
                    if src_ch == 0 {
                        continue;
                    }
                    let frames_left = (total - start) / src_ch;
                    let frames_to_write = available.min(frames_left);
                    let byte_count = frames_to_write * bytes_per_frame;
                    let mut buf = vec![0u8; byte_count];

                    let vol_norm = {
                        let st = state.lock().unwrap();
                        let v = st.volume;
                        if v > 1.0 { v / 100.0 } else { v }
                    }
                    .clamp(0.0, 1.0);

                    let src_sr = (decoded.sample_rate as f64).max(1.0);
                    let hw_sr = (current_stream_sr as f64).max(1.0);
                    let step = src_sr / hw_sr;

                    let total_frames = total / src_ch;
                    let current_frame = start / src_ch;
                    let mut float_frame = current_frame as f64;

                    if current_sample_type == WasapiSampleType::Float {
                        // 32-bit Float PCM (2 channels, 4 bytes per sample = 8 bytes per frame)
                        for fi in 0..frames_to_write {
                            let f0 = float_frame.floor() as usize;
                            let f1 = (f0 + 1).min(if total_frames > 0 { total_frames - 1 } else { 0 });
                            let frac = (float_frame - f0 as f64) as f32;

                            let (s_l, s_r) = if f0 < total_frames {
                                let b0 = f0 * src_ch;
                                let b1 = f1 * src_ch;
                                let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                                let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                                let l = (l0 * (1.0 - frac) + l1 * frac) * vol_norm;
                                let r = if src_ch >= 2 {
                                    let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                                    let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                                    (r0 * (1.0 - frac) + r1 * frac) * vol_norm
                                } else {
                                    l
                                };
                                (l, r)
                            } else {
                                (0.0, 0.0)
                            };

                            let pcm_l = s_l.clamp(-1.0, 1.0);
                            let pcm_r = s_r.clamp(-1.0, 1.0);

                            let off = fi * 8;
                            if off + 8 <= buf.len() {
                                buf[off..off + 4].copy_from_slice(&pcm_l.to_le_bytes());
                                buf[off + 4..off + 8].copy_from_slice(&pcm_r.to_le_bytes());
                            }

                            float_frame += step;
                        }
                    } else if current_container_bits == 16 {
                        // 16-bit PCM (2 channels, 2 bytes per sample = 4 bytes per frame)
                        for fi in 0..frames_to_write {
                            let f0 = float_frame.floor() as usize;
                            let f1 = (f0 + 1).min(if total_frames > 0 { total_frames - 1 } else { 0 });
                            let frac = (float_frame - f0 as f64) as f32;

                            let (s_l, s_r) = if f0 < total_frames {
                                let b0 = f0 * src_ch;
                                let b1 = f1 * src_ch;
                                let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                                let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                                let l = (l0 * (1.0 - frac) + l1 * frac) * vol_norm;
                                let r = if src_ch >= 2 {
                                    let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                                    let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                                    (r0 * (1.0 - frac) + r1 * frac) * vol_norm
                                } else {
                                    l
                                };
                                (l, r)
                            } else {
                                (0.0, 0.0)
                            };

                            let pcm_l = (s_l.clamp(-1.0, 1.0) * 32_767.0) as i16;
                            let pcm_r = (s_r.clamp(-1.0, 1.0) * 32_767.0) as i16;

                            let off = fi * 4;
                            if off + 4 <= buf.len() {
                                buf[off..off + 2].copy_from_slice(&pcm_l.to_le_bytes());
                                buf[off + 2..off + 4].copy_from_slice(&pcm_r.to_le_bytes());
                            }

                            float_frame += step;
                        }
                    } else {
                        // 24-in-32 bit Int PCM (2 channels, 4 bytes per sample = 8 bytes per frame)
                        for fi in 0..frames_to_write {
                            let f0 = float_frame.floor() as usize;
                            let f1 = (f0 + 1).min(if total_frames > 0 { total_frames - 1 } else { 0 });
                            let frac = (float_frame - f0 as f64) as f32;

                            let (s_l, s_r) = if f0 < total_frames {
                                let b0 = f0 * src_ch;
                                let b1 = f1 * src_ch;
                                let l0 = decoded.samples.get(b0).copied().unwrap_or(0.0);
                                let l1 = decoded.samples.get(b1).copied().unwrap_or(0.0);
                                let l = (l0 * (1.0 - frac) + l1 * frac) * vol_norm;
                                let r = if src_ch >= 2 {
                                    let r0 = decoded.samples.get(b0 + 1).copied().unwrap_or(0.0);
                                    let r1 = decoded.samples.get(b1 + 1).copied().unwrap_or(0.0);
                                    (r0 * (1.0 - frac) + r1 * frac) * vol_norm
                                } else {
                                    l
                                };
                                (l, r)
                            } else {
                                (0.0, 0.0)
                            };

                            let pcm_l = ((s_l.clamp(-1.0, 1.0) * 8_388_607.0) as i32) << 8;
                            let pcm_r = ((s_r.clamp(-1.0, 1.0) * 8_388_607.0) as i32) << 8;

                            let off = fi * 8;
                            if off + 8 <= buf.len() {
                                buf[off..off + 4].copy_from_slice(&pcm_l.to_le_bytes());
                                buf[off + 4..off + 8].copy_from_slice(&pcm_r.to_le_bytes());
                            }

                            float_frame += step;
                        }
                    }

                    let final_frame = float_frame.floor() as usize;
                    sample_index.store(final_frame * src_ch, Ordering::SeqCst);
                    // Update position
                    if decoded.sample_rate > 0 {
                        let mut st = state.lock().unwrap();
                        st.current_position = final_frame as f64 / decoded.sample_rate as f64;
                    }
                    buf
                } else {
                    continue;
                }
            };

            if !pcm.is_empty() {
                let frames = pcm.len() / bytes_per_frame;
                let _ = render.write_to_device(frames, &pcm, None);
            }
        }

        println!(" 🔓 WASAPI 0.23 exclusive engine exited");
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn run_wasapi_exclusive_loop(
        _state: Arc<Mutex<PlayerState>>,
        _audio_data: Arc<Mutex<Option<DecodedAudio>>>,
        _sample_index: Arc<Mutex<usize>>,
    ) -> Result<(), String> {
        Err("WASAPI Exclusive mode is only supported on Windows".to_string())
    }

    pub fn reopen_shared_stream(&self) {
        let host = cpal::default_host();
        if let Some(device) = host.default_output_device() {
            if let Ok(supported_config) = device.default_output_config() {
                let sample_format = supported_config.sample_format();
                let mut config: cpal::StreamConfig = supported_config.into();
                config.buffer_size = cpal::BufferSize::Fixed(2048);
                {
                    let mut st = self.state.lock().unwrap();
                    st.target_sample_rate = config.sample_rate.0;
                    st.target_channels = config.channels;
                }

                let state_clone = Arc::clone(&self.state);
                let audio_clone = Arc::clone(&self.audio_data);
                let idx_clone = Arc::clone(&self.sample_index);

                let err_fn = move |err| {
                    eprintln!("Audio output stream error: {}", err);
                };

                let stream_res = match sample_format {
                    SampleFormat::F32 => device.build_output_stream(
                        &config,
                        move |data: &mut [f32], _| {
                            Self::render_samples_f32(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    SampleFormat::I16 => device.build_output_stream(
                        &config,
                        move |data: &mut [i16], _| {
                            Self::render_samples_i16(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    SampleFormat::U16 => device.build_output_stream(
                        &config,
                        move |data: &mut [u16], _| {
                            Self::render_samples_u16(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                };

                if let Ok(s) = stream_res {
                    if let Ok(_) = s.play() {
                        let mut stream_guard = self._stream.lock().unwrap();
                        *stream_guard = Some(s);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn try_wasapi_exclusive_lock(_device_name: &str) -> Result<(u32, u16), String> {
        let _ = wasapi::initialize_mta();
        let formats = probe_exclusive_formats();
        if let Some(f) = formats.first() {
            return Ok((f.sr as u32, 2));
        }
        Err("Exclusive mode rejected by device — no compatible format found".to_string())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn try_wasapi_exclusive_lock(_device_name: &str) -> Result<(u32, u16), String> {
        Err("WASAPI Exclusive mode is only supported on Windows".to_string())
    }

    pub fn seek(&self, pos_secs: f64) {
        let audio_guard = self.audio_data.read().unwrap();
        let mut st = self.state.lock().unwrap();

        if let Some(ref decoded) = *audio_guard {
            if decoded.sample_rate > 0 && decoded.channels > 0 {
                let target_sample =
                    (pos_secs * decoded.sample_rate as f64 * decoded.channels as f64) as usize;
                let target_sample = target_sample.min(decoded.samples.len());

                let frame_size = decoded.channels as usize;
                let aligned_sample = (target_sample / frame_size) * frame_size;

                self.sample_index.store(aligned_sample, Ordering::SeqCst);
                st.current_position = pos_secs;
                st.is_playing = true;
                println!(
                    " ⏩ Rust Native Seek to {:.2}s (sample offset: {})",
                    pos_secs, aligned_sample
                );
            }
        }
    }

    pub fn set_audio_device(&self, device_name_or_default: &str) {
        let host = cpal::default_host();
        let mut selected_dev = None;

        if device_name_or_default == "default" || device_name_or_default.is_empty() {
            selected_dev = host.default_output_device();
        } else if let Ok(devices) = host.output_devices() {
            let clean_name = device_name_or_default
                .split(" (")
                .next()
                .unwrap_or(device_name_or_default);
            for dev in devices {
                if let Ok(name) = dev.name() {
                    if name == device_name_or_default
                        || name == clean_name
                        || device_name_or_default.starts_with(&name)
                        || name.starts_with(clean_name)
                    {
                        selected_dev = Some(dev);
                        break;
                    }
                }
            }
        }

        if selected_dev.is_none() {
            selected_dev = host.default_output_device();
        }

        if let Some(device) = selected_dev {
            if let Ok(supported_config) = device.default_output_config() {
                let sample_format = supported_config.sample_format();
                let config: cpal::StreamConfig = supported_config.into();
                {
                    let mut st = self.state.lock().unwrap();
                    st.target_sample_rate = config.sample_rate.0;
                    st.target_channels = config.channels;
                }

                let state_clone = Arc::clone(&self.state);
                let audio_clone = Arc::clone(&self.audio_data);
                let idx_clone = Arc::clone(&self.sample_index);

                let err_fn = move |err| {
                    eprintln!("Audio output stream error: {}", err);
                };

                let stream_res = match sample_format {
                    SampleFormat::F32 => device.build_output_stream(
                        &config,
                        move |data: &mut [f32], _| {
                            Self::render_samples_f32(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    SampleFormat::I16 => device.build_output_stream(
                        &config,
                        move |data: &mut [i16], _| {
                            Self::render_samples_i16(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    SampleFormat::U16 => device.build_output_stream(
                        &config,
                        move |data: &mut [u16], _| {
                            Self::render_samples_u16(data, &state_clone, &audio_clone, &idx_clone);
                        },
                        err_fn,
                        None,
                    ),
                    _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                };

                if let Ok(s) = stream_res {
                    if let Ok(_) = s.play() {
                        let mut stream_guard = self._stream.lock().unwrap();
                        *stream_guard = Some(s);
                        println!(
                            " 🔊 Switched Audio Device to '{:?}' ({} Hz)",
                            device_name_or_default, config.sample_rate.0
                        );
                    }
                }
            }
        }
    }
}

// ── Exclusive-mode helper types & functions ───────────────────────────────────

#[cfg(target_os = "windows")]
/// A format that the device confirmed works in exclusive mode.
#[derive(Debug, Clone)]
pub struct WorkingFmt {
    pub wave_fmt: WaveFormat,
    pub sr: usize,
    pub bits: usize,
    pub valid_bits: usize,
    pub sample_type: WasapiSampleType,
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug, Clone)]
pub struct WorkingFmt {
    pub sr: usize,
    pub bits: usize,
    pub valid_bits: usize,
}

#[cfg(target_os = "windows")]
static PROBED_FORMATS: std::sync::OnceLock<Vec<WorkingFmt>> = std::sync::OnceLock::new();

#[cfg(target_os = "windows")]
/// Probe the default render device and return every exclusive format it accepts.
/// Cached once at startup to avoid repeated 20-second driver initialization delays.
pub fn probe_exclusive_formats() -> Vec<WorkingFmt> {
    PROBED_FORMATS
        .get_or_init(|| {
            println!(" 🔬 Probing WASAPI exclusive formats (one-time hardware initialization)...");
            let enumerator = match DeviceEnumerator::new() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!(" ⚠️  DeviceEnumerator::new failed: {}", e);
                    return vec![];
                }
            };
            let _device = match enumerator.get_default_device(&WasapiDirection::Render) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!(" ⚠️  get_default_device failed: {}", e);
                    return vec![];
                }
            };

            // Candidates: (sr, container_bits, valid_bits, sample_type)
            let candidates: &[(usize, usize, usize, WasapiSampleType)] = &[
                // 44.1 kHz (CD Quality / Standard FLAC)
                (44100, 16, 16, WasapiSampleType::Int),
                (44100, 32, 24, WasapiSampleType::Int),
                (44100, 32, 32, WasapiSampleType::Float),
                // 48.0 kHz (Studio Standard)
                (48000, 16, 16, WasapiSampleType::Int),
                (48000, 32, 24, WasapiSampleType::Int),
                (48000, 32, 32, WasapiSampleType::Float),
                // 88.2 kHz (Hi-Res 2x CD)
                (88200, 32, 24, WasapiSampleType::Int),
                (88200, 16, 16, WasapiSampleType::Int),
                (88200, 32, 32, WasapiSampleType::Float),
                // 96.0 kHz (Hi-Res Studio)
                (96000, 32, 24, WasapiSampleType::Int),
                (96000, 16, 16, WasapiSampleType::Int),
                (96000, 32, 32, WasapiSampleType::Float),
                // 176.4 kHz (Ultra Hi-Res 4x CD)
                (176400, 32, 24, WasapiSampleType::Int),
                (176400, 16, 16, WasapiSampleType::Int),
                // 192.0 kHz (Ultra Hi-Res 4x Studio)
                (192000, 32, 24, WasapiSampleType::Int),
                (192000, 16, 16, WasapiSampleType::Int),
                (192000, 32, 32, WasapiSampleType::Float),
                // 352.8 kHz / 384.0 kHz (DXD Master)
                (352800, 32, 24, WasapiSampleType::Int),
                (384000, 32, 24, WasapiSampleType::Int),
            ];

            let mut working: Vec<WorkingFmt> = Vec::new();

            for &(sr, bits, vbits, ref stype) in candidates {
                let dev = match enumerator.get_default_device(&WasapiDirection::Render) {
                    Ok(d) => d,
                    Err(_) => continue,
                };
                let mut client = match dev.get_iaudioclient() {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                let requested = WaveFormat::new(bits, vbits, stype, sr, 2, None);
                let approved = match client.is_supported_exclusive_with_quirks(&requested) {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                let period =
                    match client.calculate_aligned_period_near(100_000, Some(128), &approved) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };

                let mode = StreamMode::EventsExclusive { period_hns: period };
                if client
                    .initialize_client(&approved, &WasapiDirection::Render, &mode)
                    .is_ok()
                {
                    working.push(WorkingFmt {
                        wave_fmt: approved,
                        sr,
                        bits,
                        valid_bits: vbits,
                        sample_type: *stype,
                    });
                }
            }

            println!(
                " 🔬 probe_exclusive_formats: {} working formats cached",
                working.len()
            );
            working
        })
        .clone()
}

#[cfg(not(target_os = "windows"))]
pub fn probe_exclusive_formats() -> Vec<WorkingFmt> {
    vec![]
}

#[cfg(target_os = "windows")]
/// Open a fresh exclusive event-driven stream for `fmt`.
/// Returns (AudioClient, AudioRenderClient, Handle, bytes_per_frame).
pub fn open_exclusive_stream(
    fmt: &WorkingFmt,
) -> Result<
    (
        wasapi::AudioClient,
        wasapi::AudioRenderClient,
        wasapi::Handle,
        usize,
    ),
    String,
> {
    let enumerator = DeviceEnumerator::new().map_err(|e| e.to_string())?;
    let device = enumerator
        .get_default_device(&WasapiDirection::Render)
        .map_err(|e| e.to_string())?;
    let mut client = device.get_iaudioclient().map_err(|e| e.to_string())?;

    let period = client
        .calculate_aligned_period_near(250_000, Some(128), &fmt.wave_fmt)
        .map_err(|e| e.to_string())?;

    let mode = StreamMode::EventsExclusive { period_hns: period };
    client
        .initialize_client(&fmt.wave_fmt, &WasapiDirection::Render, &mode)
        .map_err(|e| e.to_string())?;

    let h_event = client.set_get_eventhandle().map_err(|e| e.to_string())?;
    let render = client.get_audiorenderclient().map_err(|e| e.to_string())?;
    let block_align = fmt.wave_fmt.get_blockalign() as usize;

    Ok((client, render, h_event, block_align))
}
