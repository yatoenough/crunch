mod chorus;
mod delay;
mod distortion;
mod peak_limiter;
mod rms_normalizer;

pub use {
    chorus::Chorus, delay::Delay, distortion::Distortion, peak_limiter::PeakLimiter,
    rms_normalizer::RmsNormalizer,
};

pub trait Effect {
    fn process(&mut self, input_sample: f32) -> f32;
}
