#![allow(uncommon_codepoints)]

use std::path::Path;


use anyhow::{Ok, Result};

use crate::utils::text_parser::{parse_json, sort_manifest_by_duration, prepare_text_batches};
use crate::utils::wav_parser;
mod nn;

mod utils;
fn main() -> Result<()> {
    let mut parsed_data = parse_json(Path::new("train_data/manifest.jsonl"))?;
    sort_manifest_by_duration(&mut parsed_data);
    let batches = prepare_text_batches(&mut parsed_data);
    println!("Vec: {:?}",batches);
    //let parsed_data = wav_parser::wav_to_raw_data(Path::new("train_data/crowd/0/00a0accd6e738b7b7feed2676217c74c.wav"))?;
    //let mel_data = wav_parser::raw_data_to_spectrogram(parsed_data, 500)?;
    //
    //for item in mel_data.0.iter(){
    //    println!("{:?}", item)
    //}
    Ok(())
}
