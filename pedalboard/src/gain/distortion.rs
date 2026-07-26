use crate::{Effect, EffectParam};

#[derive(Clone)]
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
    fn name(&self) -> &'static str {
        "Distortion"
    }

    fn params(&self) -> Vec<EffectParam> {
        vec![
            EffectParam {
                name: "Drive",
                value: self.drive,
                min: 0.0,
                max: 30.0,
                step: 0.5,
            },
            EffectParam {
                name: "Threshold",
                value: self.threshold,
                min: 0.01,
                max: 1.0,
                step: 0.05,
            },
        ]
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "Drive" => self.drive = value.clamp(0.0, 30.0),
            "Threshold" => self.threshold = value.clamp(0.01, 1.0),
            _ => {}
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        let driven = sample * self.drive;
        driven.clamp(-self.threshold, self.threshold)
    }
}
