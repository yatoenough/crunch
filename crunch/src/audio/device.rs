use cpal::{StreamConfig, SupportedStreamConfig};

pub fn config_with_min_buffer(cfg: SupportedStreamConfig) -> StreamConfig {
    let mut config: StreamConfig = cfg.config();

    config.buffer_size = match cfg.buffer_size() {
        cpal::SupportedBufferSize::Range { min, .. } => cpal::BufferSize::Fixed(*min),
        cpal::SupportedBufferSize::Unknown => cpal::BufferSize::Default,
    };

    config
}
