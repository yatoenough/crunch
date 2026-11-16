mod cli;
mod engine;

use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{
    BufferSize, SampleFormat, StreamConfig, SupportedBufferSize, SupportedStreamConfigRange,
};
use neural_amp_modeler_rs::prelude::*;
use std::path::Path;
use std::time::Duration;

use crate::cli::CrunchArgs;
use crate::engine::Engine;
use crate::engine::{BLOCK, MODEL_SR};

fn pick_config(configs: impl Iterator<Item = SupportedStreamConfigRange>) -> Option<StreamConfig> {
    let c = configs
        .filter(|c| c.sample_format() == SampleFormat::F32)
        .find_map(|c| c.try_with_sample_rate(MODEL_SR))?;
    let buffer_size = match *c.buffer_size() {
        SupportedBufferSize::Range { min, max } if (min..=max).contains(&(BLOCK as u32)) => {
            BufferSize::Fixed(BLOCK as u32)
        }
        _ => BufferSize::Default,
    };
    Some(StreamConfig {
        channels: c.channels(),
        sample_rate: MODEL_SR,
        buffer_size,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CrunchArgs::parse();
    let nam = Path::new(&args.nam);

    if !nam.exists() {
        panic!("file not found");
    } else if !nam.is_file() {
        panic!("{} is not a file", args.nam);
    }

    let sys = SystemSnapshot::capture();
    let mut model_pair = load_and_build_model(nam, &sys, false, LoadOptions::default())?;
    let mut model = model_pair.model_l.take().ok_or("model_l missing")?;
    model.reset(MODEL_SR, BLOCK)?;

    let host = cpal::default_host();
    let (in_dev, in_cfg) = cli::choose_device(
        "input",
        host.input_devices()?,
        host.default_input_device(),
        |d| pick_config(d.supported_input_configs().ok()?),
    )?;
    let (out_dev, out_cfg) = cli::choose_device(
        "output",
        host.output_devices()?,
        host.default_output_device(),
        |d| pick_config(d.supported_output_configs().ok()?),
    )?;
    println!("in:  {} {in_cfg:?}", in_dev.id()?);
    println!("out: {} {out_cfg:?}", out_dev.id()?);

    let engine = Engine::start(&in_dev, in_cfg, &out_dev, out_cfg, model)?;
    println!("running, ctrl-c to stop");

    loop {
        std::thread::sleep(Duration::from_secs(1));
        let (o, u) = engine.take_xruns();
        if o + u > 0 {
            eprintln!("dropped {o} samples (output behind), {u} silent samples (input behind)");
        }
    }
}
