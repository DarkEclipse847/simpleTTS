use std::path::PathBuf;
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
