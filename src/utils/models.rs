use std::path::PathBuf;
use candle_core::Tensor;
use serde::Deserialize;
use std::path::Path;


use crate::utils::wav_parser::wav_duration;
pub struct Dataset{
    samples: Vec<ManifestItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "RawCSV")]
pub struct ManifestItem{
    pub id: String,
    pub audio_filepath: PathBuf,
    pub text: String,
    pub normalized_text: Option<String>,
    pub duration: f32,
}

#[derive(Deserialize)]
pub struct RawCSV{
    pub id: String,
    pub text: String,
    pub normalized_text: Option<String>,
}

// TODO: remove hardcoded filepath
impl TryFrom<RawCSV> for ManifestItem{
    type Error = anyhow::Error;
    fn try_from(raw: RawCSV) -> Result<Self, Self::Error> {
        let audio_path: PathBuf = PathBuf::from(format!("train_data/ruslan_dataset/data/{}.wav", raw.id));
        let duration = wav_duration(Path::new(&audio_path))?;
        Ok(ManifestItem {
            id: raw.id,
            audio_filepath: audio_path,
            text: raw.text,
            normalized_text: raw.normalized_text,
            duration: duration
        })
    }
}

pub struct Batch{
    pub audio: Tensor,
    pub text: Tensor,
    pub mel_length: Tensor,
}