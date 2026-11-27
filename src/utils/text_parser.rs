use std::path::Path;
use anyhow::{Error, Ok, Result};
use indicatif::{ProgressBar, ProgressFinish::Abandon, ProgressIterator, ProgressStyle};
use rand::seq::SliceRandom;
use serde_jsonlines::json_lines;
use std::thread;
use std::time::Duration;
use std::fs;

use crate::utils::models::{self, ManifestItem, RawCSV};

//////////////////////////////////
// JSONL for NeMo-like datasets //
//////////////////////////////////

pub fn parse_jsonl(path: &Path) -> Result<Vec<ManifestItem>, Error>{
    // TODO: add a dynamic path to directory
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


/////////////////////////////////
// CSV for Piper-like datasets //
/////////////////////////////////

pub fn parse_csv(path: &Path, data_path: &Path) -> Result<Vec<ManifestItem>, Error>{

    // misc for beauty
    println!("/////////////////////////////////");
    println!("// Starting to process dataset //");
    println!("/////////////////////////////////\n");
    thread::sleep(Duration::from_secs(3));
    let count = fs::read_dir(data_path)?.count();
    let bar = ProgressBar::new(count as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({per_sec}, {eta})"
        ).unwrap().progress_chars("█▒░"),
    );

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_path(path)?;
    let parsed_data_csv: Vec<ManifestItem> = csv_reader
        .deserialize::<RawCSV>()
        .progress_with(bar.with_finish(Abandon))
        .map(|raw| ManifestItem::try_from_raw(raw?, &data_path))
        .collect::<Result<_, _>>()?; 
    Ok(parsed_data_csv)
}