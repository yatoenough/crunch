use clap::Parser;
use hound::{WavReader, WavWriter};
use pedalboard::{
    Effect, EffectChain,
    gain::Distortion,
    modulation::{Chorus, Delay},
};

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
    std::fs::create_dir_all("testdata").unwrap();
    let args = Args::parse();

    let mut reader = WavReader::open(&args.input).unwrap();
    let spec = reader.spec();
    let mut writer = WavWriter::create(&args.output, spec).unwrap();
    let sample_rate = spec.sample_rate as f32;

    let mut chain = EffectChain::new()
        .apply(Distortion::new(10.0, 1.0))
        .apply(Chorus::new(sample_rate))
        .apply(Delay::new(sample_rate, 100.0, 0.5, 0.5, 1.0));

    let scale = i16::MAX as f32;
    for sample in reader.samples::<i16>() {
        let s = sample.unwrap() as f32 / scale;
        let processed = chain.process(s);
        writer.write_sample((processed * scale) as i16).unwrap();
    }

    writer.finalize().unwrap();
}
