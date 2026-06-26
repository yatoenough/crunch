use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use pedalboard::{
    EffectChain,
    gain::Overdrive,
    modulation::{Chorus, Delay},
};

pub mod audio;
pub mod engine;

use crate::{audio::selector::pick_device, engine::AudioEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let sample_rate = 48000.0;

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

    let input_config = audio::device::config_with_min_buffer(input_device.default_input_config()?);
    let output_config =
        audio::device::config_with_min_buffer(output_device.default_output_config()?);

    let effects = Arc::new(Mutex::new(EffectChain::new(vec![
        Box::new(Overdrive::new(15.0, 0.5)),
        Box::new(Chorus::new(sample_rate)),
        Box::new(Delay::new(sample_rate, 100.0, 0.1, 0.2, 0.43)),
    ])));

    let engine = AudioEngine::new(effects, input_config, output_config, sample_rate);

    let input_stream = input_device.build_input_stream(
        input_config,
        engine.input_callback(),
        |e| eprintln!("{e}"),
        None,
    )?;

    let output_stream = output_device.build_output_stream(
        output_config,
        engine.output_callback(),
        |e| eprintln!("{e}"),
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}
