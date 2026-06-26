use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use guitar_processor::{
    audio::{self, pipeline::EffectChain, selector::pick_device},
    effects::{Chorus, Delay, Distortion, PeakLimiter, RmsNormalizer},
    engine::AudioEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    let input_device = pick_device(
        "input",
        host.input_devices()?.collect(),
        host.default_input_device(),
    )?;

    let output_device = pick_device(
        "output",
        host.output_devices()?.collect(),
        host.default_output_device(),
    )?;

    let input_cfg = audio::device::config_with_min_buffer(input_device.default_input_config()?);
    let output_cfg = audio::device::config_with_min_buffer(output_device.default_output_config()?);

    let engine = AudioEngine {
        buffer: Arc::new(Mutex::new(VecDeque::new())),
        effects: Arc::new(Mutex::new(EffectChain::new(vec![
            Box::new(RmsNormalizer::new(48000.0)),
            Box::new(PeakLimiter::new(48000.0)),
            Box::new(Distortion::new(15.0, 0.5)),
            Box::new(Chorus::new(48000.0)),
            Box::new(Delay::new(48000.0, 100.0, 0.1, 0.2, 0.43)),
        ]))),
        input_channels: input_cfg.channels as usize,
        output_channels: output_cfg.channels as usize,
        sample_rate: 48000.0,
    };

    let input_stream = input_device.build_input_stream(
        input_cfg,
        engine.input_callback(),
        |e| eprintln!("{e}"),
        None,
    )?;

    let output_stream = output_device.build_output_stream(
        output_cfg,
        engine.output_callback(),
        |e| eprintln!("{e}"),
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}
