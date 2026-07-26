use crate::{Effect, EffectParam};

#[derive(Clone)]
pub struct Fuzz {
    drive: f32,
    threshold: f32,
}

impl Fuzz {
    pub fn new(drive: f32, threshold: f32) -> Self {
        Self { drive, threshold }
    }
}

impl Effect for Fuzz {
    fn name(&self) -> &'static str {
        "Fuzz"
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
        let mut driven = sample * self.drive;

        while driven > self.threshold || driven < -self.threshold {
            if driven > self.threshold {
                driven = 2.0 * self.threshold - driven;
            }
            if driven < -self.threshold {
                driven = -2.0 * self.threshold - driven;
            }
        }
        driven
    }
}
