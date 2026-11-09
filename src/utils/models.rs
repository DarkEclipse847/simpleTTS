use std::path::PathBuf;
use serde::Deserialize;

pub struct MetadataSample{
    audio_path: PathBuf,
    text_tokens: Vec<u32>,
}
pub struct Dataset{
    samples: Vec<MetadataSample>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManifestItem{
    pub id: String,
    pub audio_filepath: PathBuf,
    pub text: String,
}