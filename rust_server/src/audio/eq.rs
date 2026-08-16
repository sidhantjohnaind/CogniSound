use std::f32::consts::PI;

pub const EQ_FREQUENCIES: [f32; 10] = [
    31.25, 62.5, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

#[derive(Clone, Copy, Debug)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl BiquadCoeffs {
    pub fn peaking_eq(center_freq: f32, sample_rate: f32, db_gain: f32, q: f32) -> Self {
        if db_gain.abs() < 0.01 {
            return Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 };
        }

        let w0 = 2.0 * PI * (center_freq / sample_rate).clamp(0.0001, 0.499);
        let alpha = w0.sin() / (2.0 * q);
        let a = 10.0f32.powf(db_gain / 40.0);
        let cos_w0 = w0.cos();

        let b0_raw = 1.0 + alpha * a;
        let b1_raw = -2.0 * cos_w0;
        let b2_raw = 1.0 - alpha * a;
        let a0_raw = 1.0 + alpha / a;
        let a1_raw = -2.0 * cos_w0;
        let a2_raw = 1.0 - alpha / a;

        Self {
            b0: b0_raw / a0_raw,
            b1: b1_raw / a0_raw,
            b2: b2_raw / a0_raw,
            a1: a1_raw / a0_raw,
            a2: a2_raw / a0_raw,
        }
    }
}

#[derive(Clone, Debug, Default, Copy)]
pub struct BiquadState {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl BiquadState {
    #[inline(always)]
    pub fn process(&mut self, input: f32, c: &BiquadCoeffs) -> f32 {
        let output = c.b0 * input + c.b1 * self.x1 + c.b2 * self.x2 - c.a1 * self.y1 - c.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }
}

#[derive(Clone, Debug)]
pub struct TenBandEq {
    pub coeffs: [BiquadCoeffs; 10],
    pub state_l: [BiquadState; 10],
    pub state_r: [BiquadState; 10],
    pub enabled: bool,
    pub sample_rate: f32,
    pub gains: [f32; 10],
}

impl TenBandEq {
    pub fn new(sample_rate: f32) -> Self {
        let mut eq = Self {
            coeffs: [BiquadCoeffs { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 }; 10],
            state_l: [BiquadState::default(); 10],
            state_r: [BiquadState::default(); 10],
            enabled: false,
            sample_rate,
            gains: [0.0; 10],
        };
        eq.update_gains([0.0; 10], false);
        eq
    }

    pub fn update_gains(&mut self, gains: [f32; 10], enabled: bool) {
        self.gains = gains;
        self.enabled = enabled;
        let is_any_gain = gains.iter().any(|g| g.abs() > 0.01);
        if !enabled || !is_any_gain {
            for i in 0..10 {
                self.coeffs[i] = BiquadCoeffs { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 };
            }
            return;
        }

        let q = 1.414;
        for i in 0..10 {
            self.coeffs[i] = BiquadCoeffs::peaking_eq(EQ_FREQUENCIES[i], self.sample_rate, gains[i], q);
        }
    }

    #[inline(always)]
    pub fn process_sample(&mut self, left: f32, right: f32) -> (f32, f32) {
        if !self.enabled {
            return (left, right);
        }

        let mut l = left;
        let mut r = right;

        for i in 0..10 {
            if self.gains[i].abs() > 0.01 {
                l = self.state_l[i].process(l, &self.coeffs[i]);
                r = self.state_r[i].process(r, &self.coeffs[i]);
            }
        }

        (l, r)
    }
}

pub const EQ_FREQUENCIES_15: [f32; 15] = [
    25.0, 40.0, 63.0, 100.0, 160.0, 250.0, 400.0, 630.0, 1000.0, 1600.0, 2500.0, 4000.0, 6300.0, 10000.0, 16000.0,
];

#[derive(Clone, Debug)]
pub struct FifteenBandEq {
    pub coeffs: [BiquadCoeffs; 15],
    pub state_l: [BiquadState; 15],
    pub state_r: [BiquadState; 15],
    pub enabled: bool,
    pub sample_rate: f32,
    pub gains: [f32; 15],
}

impl FifteenBandEq {
    pub fn new(sample_rate: f32) -> Self {
        let mut eq = Self {
            coeffs: [BiquadCoeffs { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 }; 15],
            state_l: [BiquadState::default(); 15],
            state_r: [BiquadState::default(); 15],
            enabled: false,
            sample_rate,
            gains: [0.0; 15],
        };
        eq.update_gains([0.0; 15], false);
        eq
    }

    pub fn update_gains(&mut self, gains: [f32; 15], enabled: bool) {
        self.gains = gains;
        self.enabled = enabled;
        let is_any_gain = gains.iter().any(|g| g.abs() > 0.01);
        if !enabled || !is_any_gain {
            for i in 0..15 {
                self.coeffs[i] = BiquadCoeffs { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 };
            }
            return;
        }

        let q = 1.414;
        for i in 0..15 {
            self.coeffs[i] = BiquadCoeffs::peaking_eq(EQ_FREQUENCIES_15[i], self.sample_rate, gains[i], q);
        }
    }

    #[inline(always)]
    pub fn process_sample(&mut self, left: f32, right: f32) -> (f32, f32) {
        if !self.enabled {
            return (left, right);
        }

        let mut l = left;
        let mut r = right;

        for i in 0..15 {
            if self.gains[i].abs() > 0.01 {
                l = self.state_l[i].process(l, &self.coeffs[i]);
                r = self.state_r[i].process(r, &self.coeffs[i]);
            }
        }

        (l, r)
    }
}

// --- Bauer BS2B Headphone Crossfeed Filter (MusicBee DSP Engine) ---
#[derive(Clone, Debug)]
pub struct Bs2bCrossfeed {
    pub enabled: bool,
    pub level_db: f32,
    pub lo_l: f32,
    pub lo_r: f32,
}

impl Default for Bs2bCrossfeed {
    fn default() -> Self {
        Self::new()
    }
}

impl Bs2bCrossfeed {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level_db: -4.5,
            lo_l: 0.0,
            lo_r: 0.0,
        }
    }

    #[inline(always)]
    pub fn process_sample(&mut self, left: f32, right: f32) -> (f32, f32) {
        if !self.enabled {
            return (left, right);
        }
        // Chu Moy / Bauer BS2B 700Hz low-pass crossfeed matrix
        let feed_factor = 0.22; // ~-4.5dB crossfeed
        let alpha = 0.15; // ~700Hz cut-off filter coefficient

        self.lo_l += alpha * (left - self.lo_l);
        self.lo_r += alpha * (right - self.lo_r);

        let out_l = left + feed_factor * self.lo_r;
        let out_r = right + feed_factor * self.lo_l;
        (out_l, out_r)
    }
}
