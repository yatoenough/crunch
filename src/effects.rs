mod chorus;
mod delay;
mod overdrive;
mod peak_limiter;
mod rms_normalizer;

pub use {
    chorus::Chorus, delay::Delay, overdrive::Overdrive, peak_limiter::PeakLimiter,
    rms_normalizer::RmsNormalizer,
};

pub trait Effect {
    fn process(&mut self, input_sample: f32) -> f32;
}
