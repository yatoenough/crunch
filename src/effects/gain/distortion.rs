use crate::effects::Effect;

pub struct Distortion {
    drive: f32,
    threshold: f32,
}

impl Distortion {
    pub fn new(drive: f32, threshold: f32) -> Self {
        Self { drive, threshold }
    }
}

impl Effect for Distortion {
    fn process(&mut self, input_sample: f32) -> f32 {
        let driven = input_sample * self.drive;
        driven.clamp(-self.threshold, self.threshold)
    }
}
