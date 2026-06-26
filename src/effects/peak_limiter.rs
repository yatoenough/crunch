use crate::effects::Effect;

pub struct PeakLimiter {
    peak: f32,
    attack: f32,
    release: f32,
    ceiling: f32,
}

impl PeakLimiter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            peak: 1.0,
            attack: 1.0 - (-2.2 / (0.01 * sample_rate)).exp(),
            release: 1.0 - (-2.2 / (0.3 * sample_rate)).exp(),
            ceiling: 0.95,
        }
    }
}

impl Effect for PeakLimiter {
    fn process(&mut self, sample: f32) -> f32 {
        let abs = sample.abs();
        if abs > self.peak {
            self.peak += self.attack * (abs - self.peak);
        } else {
            self.peak += self.release * (abs - self.peak);
        }

        let gain = self.ceiling / self.peak.max(self.ceiling);
        sample * gain
    }
}
