use std::path::Path;
use anyhow::{Error, Ok, Result};
use rand::seq::SliceRandom;
use serde_jsonlines::json_lines;

use crate::utils::models::{self, ManifestItem};

pub fn parse_json(path: &Path) -> Result<Vec<models::ManifestItem>, Error>{
    let parsed_data = json_lines(path)?
        .map(|res| res.map_err(anyhow::Error::from))
        .collect::<Result<Vec<models::ManifestItem>>>()?;
    Ok(parsed_data)
}
pub fn sort_manifest_by_duration(manifest: &mut Vec<models::ManifestItem>){
    manifest
        .sort_by(|a, b| a
            .duration.partial_cmp(&b.duration)
            .unwrap_or(std::cmp::Ordering::Equal));
}

pub fn prepare_text_batches(manifest: &mut Vec<models::ManifestItem>) -> Vec<Vec<ManifestItem>>{
    let mut batches: Vec<Vec<ManifestItem>> = manifest
        .chunks(16)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut rng = rand::thread_rng();
    batches.shuffle(&mut rng);
    return batches;
}

