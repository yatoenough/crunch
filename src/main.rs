use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SupportedBufferSize};
use guitar_processor::effects::{Chorus, Delay, Distortion, Effect, PeakLimiter, RmsNormalizer};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

fn select_device(
    label: &str,
    devices: Vec<cpal::Device>,
    default_device: Option<cpal::Device>,
) -> Result<cpal::Device, Box<dyn std::error::Error>> {
    if devices.is_empty() {
        return default_device.ok_or_else(|| format!("Could not find {} device", label).into());
    }

    let default_index = default_device
        .as_ref()
        .and_then(|default| devices.iter().position(|device| device == default));

    println!("\nAvailable {} devices:", label);
    for (index, device) in devices.iter().enumerate() {
        let default_marker = if Some(index) == default_index {
            " (default)"
        } else {
            ""
        };
        println!("  {}. {}{}", index + 1, device, default_marker);
    }

    loop {
        if let Some(index) = default_index {
            print!("Select {} device [default {}]: ", label, index + 1);
        } else {
            print!("Select {} device: ", label);
        }
        io::stdout().flush()?;

        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        let selection = selection.trim();

        if selection.is_empty()
            && let Some(index) = default_index
        {
            return Ok(devices.into_iter().nth(index).unwrap());
        }

        match selection.parse::<usize>() {
            Ok(number) if (1..=devices.len()).contains(&number) => {
                return Ok(devices.into_iter().nth(number - 1).unwrap());
            }
            _ => println!("Please enter a number from 1 to {}.", devices.len()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    let input_device = select_device(
        "input",
        host.input_devices()?.collect(),
        host.default_input_device(),
    )?;

    let output_device = select_device(
        "output",
        host.output_devices()?.collect(),
        host.default_output_device(),
    )?;

    let input_config = input_device.default_input_config()?;
    let output_config = output_device.default_output_config()?;

    let mut input_stream_config: cpal::StreamConfig = input_config.config();
    input_stream_config.buffer_size = match input_config.buffer_size() {
        SupportedBufferSize::Range { min, .. } => BufferSize::Fixed(*min),
        SupportedBufferSize::Unknown => BufferSize::Default,
    };

    let mut output_stream_config: cpal::StreamConfig = output_config.config();
    output_stream_config.buffer_size = match output_config.buffer_size() {
        SupportedBufferSize::Range { min, .. } => BufferSize::Fixed(*min),
        SupportedBufferSize::Unknown => BufferSize::Default,
    };

    println!("Using input config: {:?}", input_stream_config);
    println!("Using output config: {:?}", output_stream_config);

    let buffer = Arc::new(Mutex::new(VecDeque::new()));

    let buffer_out = Arc::clone(&buffer);
    let sample_rate = 48000_f32;
    let input_channels = input_stream_config.channels as usize;
    let output_channels = output_stream_config.channels as usize;

    let mut effects: Vec<Box<dyn Effect + Send>> = vec![
        Box::new(RmsNormalizer::new(sample_rate)),
        Box::new(PeakLimiter::new(sample_rate)),
        Box::new(Distortion::new(15.0, 0.5)),
        Box::new(Chorus::new(sample_rate)),
        Box::new(Delay::new(sample_rate, 100.0, 0.1, 0.2, 0.43)),
    ];

    let input_stream = input_device.build_input_stream(
        input_stream_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = buffer.lock().unwrap_or_else(|err| err.into_inner());
            for frame in data.chunks(input_channels) {
                let mut sample = frame.iter().sum::<f32>() / frame.len() as f32;

                for effect in &mut effects {
                    sample = effect.process(sample);
                }

                buf.push_back(sample);
            }
        },
        |err| eprintln!("Input stream error: {}", err),
        None,
    )?;

    let output_stream = output_device.build_output_stream(
        output_stream_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut buf = buffer_out.lock().unwrap_or_else(|err| err.into_inner());
            for frame in data.chunks_mut(output_channels) {
                let sample = buf.pop_front().unwrap_or(0.0);
                for channel_sample in frame {
                    *channel_sample = sample;
                }
            }
        },
        |err| eprintln!("Output stream error: {}", err),
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    println!("Rock and roll is running... Press Enter to stop.");
    let mut x = String::new();
    std::io::stdin().read_line(&mut x)?;

    Ok(())
}
