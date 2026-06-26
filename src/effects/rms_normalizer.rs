use crate::effects::Effect;

pub struct RmsNormalizer {
    rms_avg: f32,
    attack: f32,
    release: f32,
    target_rms: f32,
    max_gain: f32,
}

impl RmsNormalizer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            rms_avg: 0.01,
            attack: 1.0 - (-2.2 / (0.05 * sample_rate)).exp(),
            release: 1.0 - (-2.2 / (1.0 * sample_rate)).exp(),
            target_rms: 0.2,
            max_gain: 50.0,
        }
    }
}

impl Effect for RmsNormalizer {
    fn process(&mut self, sample: f32) -> f32 {
        let power = sample * sample;

        let coeff = if power > self.rms_avg {
            self.attack
        } else {
            self.release
        };
        self.rms_avg += coeff * (power - self.rms_avg);

        let rms = self.rms_avg.sqrt().max(1e-6);
        let gain = (self.target_rms / rms).min(self.max_gain);

        sample * gain
    }
}
