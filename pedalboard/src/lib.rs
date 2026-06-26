pub mod gain;
pub mod modulation;

pub trait Effect {
    fn process(&mut self, input_sample: f32) -> f32;
}
