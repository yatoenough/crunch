use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use neural_amp_modeler_rs::prelude::*;

pub const BLOCK: usize = 64;
pub const MODEL_SR: u32 = 48_000;
pub const PREFILL: usize = 4 * BLOCK;

pub struct Engine {
    _input: Stream,
    _output: Stream,
    overruns: Arc<AtomicUsize>,
    underruns: Arc<AtomicUsize>,
}

impl Engine {
    pub fn start(
        in_dev: &Device,
        in_cfg: StreamConfig,
        out_dev: &Device,
        out_cfg: StreamConfig,
        mut model: Box<StaticModel>,
    ) -> Result<Engine, Box<dyn std::error::Error>> {
        let (mut tx, mut rx) = rtrb::RingBuffer::<f32>::new(16 * 1024);
        for _ in 0..PREFILL {
            tx.push(0.0)?;
        }

        let overruns = Arc::new(AtomicUsize::new(0));
        let underruns = Arc::new(AtomicUsize::new(0));

        let in_ch = in_cfg.channels as usize;
        let out_ch = out_cfg.channels as usize;
        let dropped = overruns.clone();
        let (mut x, mut y) = ([0.0f32; BLOCK], [0.0f32; BLOCK]);
        let input = in_dev.build_input_stream(
            in_cfg,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for frames in data.chunks(BLOCK * in_ch) {
                    let n = frames.len() / in_ch;
                    for (s, f) in x.iter_mut().zip(frames.chunks_exact(in_ch)) {
                        *s = f[0];
                    }
                    model.process(&x[..n], &mut y[..n]);
                    for &s in &y[..n] {
                        if tx.push(s).is_err() {
                            dropped.fetch_add(1, Relaxed);
                        }
                    }
                }
            },
            |e| eprintln!("input stream error: {e}"),
            None,
        )?;

        let starved = underruns.clone();
        let output = out_dev.build_output_stream(
            out_cfg,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_exact_mut(out_ch) {
                    let s = rx.pop().unwrap_or_else(|_| {
                        starved.fetch_add(1, Relaxed);
                        0.0
                    });
                    frame.fill(s);
                }
            },
            |e| eprintln!("output stream error: {e}"),
            None,
        )?;

        input.play()?;
        output.play()?;

        Ok(Engine {
            _input: input,
            _output: output,
            overruns,
            underruns,
        })
    }

    pub fn take_xruns(&self) -> (usize, usize) {
        (
            self.overruns.swap(0, Relaxed),
            self.underruns.swap(0, Relaxed),
        )
    }
}
