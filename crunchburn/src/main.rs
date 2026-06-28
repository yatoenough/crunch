use clap::Parser;
use hound::{WavReader, WavWriter};
use pedalboard::{Effect, EffectChain, gain::Distortion, modulation::Chorus};

#[derive(Parser)]
#[command(about = "Audio effect chain processor")]
struct Args {
    /// Input WAV file
    input: String,

    /// Output WAV file
    #[arg(short, long, default_value = "output.wav")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let mut reader = WavReader::open(&args.input).unwrap();
    let spec = reader.spec();
    let mut writer = WavWriter::create(&args.output, spec).unwrap();
    let sample_rate = spec.sample_rate as f32;

    let mut chain = EffectChain::new()
        .apply(Distortion::new(3.0, 0.7))
        .apply(Chorus::new(sample_rate));

    let scale = i16::MAX as f32;
    for sample in reader.samples::<i16>() {
        let s = sample.unwrap() as f32 / scale;
        let processed = chain.process(s);
        writer.write_sample((processed * scale) as i16).unwrap();
    }

    writer.finalize().unwrap();
}
