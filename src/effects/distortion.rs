use crate::effects::Effect;

pub struct Distortion {
    gain: f32,
    threshold: f32,
}

#[deprecated(note = "this effect is experimental and needs to be refactored.")]
impl Distortion {
    pub fn new(gain: f32, threshold: f32) -> Self {
        Self { gain, threshold }
    }
}

impl Effect for Distortion {
    fn process(&mut self, input_sample: f32) -> f32 {
        let driven = input_sample * self.gain;
        let distorted = driven.tanh();

        distorted * self.threshold
    }
}
