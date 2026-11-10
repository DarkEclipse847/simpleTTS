use std::path::PathBuf;
use candle_core::Tensor;
use serde::Deserialize;
pub struct Dataset{
    samples: Vec<ManifestItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManifestItem{
    pub id: String,
    pub audio_filepath: PathBuf,
    pub text: String,
    pub duration: f32,
}

pub struct Batch{
    pub audio: Tensor,
    pub text: Tensor,
    pub mel_length: Tensor,
}