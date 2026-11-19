use std::path::PathBuf;
use candle_core::Tensor;
use serde::Deserialize;
use std::path::Path;


use crate::utils::wav_parser::wav_duration;
pub struct Dataset{
    samples: Vec<ManifestItem>,
}

#[derive(Deserialize, Debug, Clone)]
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

impl ManifestItem{
    pub fn try_from_raw(raw: RawCSV, data_dir: &Path) -> anyhow::Result<Self> {
        let audio_path: PathBuf = data_dir.join(format!("{}.wav", raw.id));
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

pub struct RussianTokenizer{
    pub alphabet: Vec<char>, 
}

impl RussianTokenizer {
    pub fn new() -> Self{
        let mut alphabet = vec!['_', '~'];
        alphabet.extend("абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars());
        alphabet.push('\u{0301}');
        alphabet.push('😡'); //anger
        alphabet.push('😭'); //sadness
        alphabet.push('😲'); //astonishment
        alphabet.push('😄'); //happiness
        alphabet.push('😨'); //fear
        alphabet.push('🤢'); //disgust
        alphabet.extend(" .,!?-\"".chars());
        Self { alphabet }
    }
}