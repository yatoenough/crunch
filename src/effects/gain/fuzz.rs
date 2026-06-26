use crate::effects::Effect;

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
    fn process(&mut self, input_sample: f32) -> f32 {
        let mut driven = input_sample * self.drive;

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
