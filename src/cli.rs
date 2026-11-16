use crate::engine::MODEL_SR;

use clap::Parser;
use cpal::traits::DeviceTrait;
use cpal::{Device, StreamConfig};

use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CrunchArgs {
    /// NAM file to use
    #[arg(short, long)]
    pub nam: String,
}

pub fn choose_device(
    kind: &str,
    devices: impl Iterator<Item = Device>,
    default: Option<Device>,
    config: impl Fn(&Device) -> Option<StreamConfig>,
) -> Result<(Device, StreamConfig), Box<dyn std::error::Error>> {
    let usable: Vec<_> = devices.filter_map(|d| config(&d).map(|c| (d, c))).collect();
    if usable.is_empty() {
        return Err(format!("no {kind} device supports f32 at {MODEL_SR} Hz").into());
    }

    let default_id = default.and_then(|d| d.id().ok());
    let default_idx = usable
        .iter()
        .position(|(d, _)| d.id().ok() == default_id)
        .unwrap_or(0);

    println!("{kind} devices:");
    for (i, (d, c)) in usable.iter().enumerate() {
        let name = d
            .description()
            .map_or_else(|_| "<unknown>".into(), |n| n.to_string());
        let mark = if i == default_idx { '*' } else { ' ' };
        println!(" {mark}{i}: {name} ({} ch)", c.channels);
    }

    loop {
        print!("{kind} [{default_idx}]: ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line)? == 0 {
            return Err("stdin closed".into());
        }
        let line = line.trim();
        let idx = if line.is_empty() {
            Some(default_idx)
        } else {
            line.parse().ok().filter(|&i| i < usable.len())
        };
        match idx {
            Some(i) => return Ok(usable.into_iter().nth(i).unwrap()),
            None => println!("enter a number from 0 to {}", usable.len() - 1),
        }
    }
}
