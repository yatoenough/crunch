# crunch

Real-time guitar effects processor right in your terminal.

`crunch` reads audio from an input device (e.g. an audio interface with your guitar plugged in), runs it through a chain of DSP effects, and writes the result to an output device — all with a live callback loop, no offline processing

*(note: i created crunchburn - a cli to process audio file with no live processing for testing purposes, because i dont have a proper audio interface yet :p).*

## Workspace layout

This is a Cargo workspace with two crates:

- **`crunch`** — the CLI binary. Handles device selection, audio I/O via [`cpal`](https://crates.io/crates/cpal), and the ring-buffer engine that connects input and output callbacks.
- **`pedalboard`** — a standalone DSP library of guitar effects, decoupled from any audio backend. Depends on nothing but `std`.

```
crunch/
├── crunch/           # CLI binary
│   └── src/
│       ├── main.rs       # device setup, effect chain wiring, stream lifecycle
│       ├── engine.rs      # AudioEngine: input/output stream callbacks
│       └── audio/
│           ├── buffer.rs     # VecDeque-backed sample ring buffer
│           ├── device.rs     # stream config helpers (min buffer size)
│           └── selector.rs   # interactive device picker
└── pedalboard/        # DSP effects library
    └── src/
        ├── lib.rs         # Effect trait, EffectChain
        ├── gain/           # Overdrive, Distortion, Fuzz
        └── modulation/     # Chorus, Delay
```

## Effects

`pedalboard` exposes a simple `Effect` trait:

```rust
pub trait Effect: Send + Sync {
    fn process(&mut self, input_sample: f32) -> f32;
}
```

Effects are composed into an `EffectChain`, which is itself an `Effect`, so chains nest freely.

| Effect | Module | Description |
|---|---|---|
| `Overdrive` | `gain::Overdrive` | Soft-clips via `tanh` saturation, scaled by drive and output volume. |
| `Distortion` | `gain::Distortion` | Hard clip at a fixed threshold after gain boost. |
| `Fuzz` | `gain::Fuzz` | Hard clip with reflection instead of flat clamping, for a more aggressive/asymmetric character. |
| `Chorus` | `modulation::Chorus` | LFO-modulated delay line with linear interpolation, mixed with dry signal. |
| `Delay` | `modulation::Delay` | Feedback delay line with configurable time, feedback, wet/dry mix. |

## How audio flows

1. `main.rs` picks an input and output device (interactively, via `selector::pick_device`), and builds `cpal` stream configs using the device's minimum supported buffer size for lowest latency.
2. An `EffectChain` is constructed and wrapped in `Arc<Mutex<_>>` so it's shared between the input callback and the (future) control surface.
3. `AudioEngine` owns a lock-protected `AudioBuffer` (a sample-level ring buffer):
   - The **input callback** downmixes each input frame to mono, runs it through the effect chain, and pushes the result into the buffer.
   - The **output callback** pops one sample per output frame and copies it to every output channel.
4. Both streams are started, and the process blocks on stdin until you hit Enter.

The effect chain currently wired up in `main.rs`:

```rust
EffectChain::new(vec![
    Box::new(Overdrive::new(15.0, 0.5)),
    Box::new(Chorus::new(sample_rate)),
    Box::new(Delay::new(sample_rate, 100.0, 0.1, 0.2, 0.43)),
])
```

## Requirements

- Rust (edition 2024 — a recent stable/nightly toolchain)
- A working audio backend supported by `cpal` (ALSA/JACK on Linux, CoreAudio on macOS, WASAPI on Windows)
- An audio input device — an interface with a guitar input, or any mic, works for testing

## Usage

```bash
cargo run --release -p crunch
```

You'll be prompted to select an input and output device:

```
Available input devices:
  1. Scarlett 2i2 USB (default)
  2. Built-in Microphone
Select input device [default 1]:
```

Press Enter to accept the default, or type a number. Once both streams are running, play — audio is processed sample-by-sample in real time. Use control hints to add/remove effects and start/stop processing.

## Extending

To add a new effect, implement `Effect` for a new struct under `pedalboard/src/gain/` or `pedalboard/src/modulation/` (or a new category module), re-export it from the category's `mod.rs`, and add it to the `EffectChain` in `main.rs`. Since `Effect` only requires `process(&mut self, f32) -> f32`, effects are trivial to unit test in isolation from any audio I/O.

## License

MIT © Nikita Shyshkin
