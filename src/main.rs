#![allow(uncommon_codepoints)]

use std::path::Path;


use anyhow::{Ok, Result};

use crate::utils::text_parser::{parse_csv, parse_jsonl, prepare_text_batches, sort_manifest_by_duration};
use crate::utils::wav_parser;
mod nn;

mod utils;
fn main() -> Result<()> {
    let mut parsed_data = parse_csv(Path::new("train_data/ruslan_dataset/metadata_RUSLAN_22200.csv"))?;
    sort_manifest_by_duration(&mut parsed_data);
    let batches = prepare_text_batches(&mut parsed_data);
    //println!("Vec: {:?}",batches);
    //let parsed_data = wav_parser::wav_to_raw_data(Path::new("train_data/crowd/0/00a0accd6e738b7b7feed2676217c74c.wav"))?;
    //let mel_data = wav_parser::raw_data_to_spectrogram(parsed_data, 500)?;
    //
    //for item in mel_data.0.iter(){
    //    println!("{:?}", item)
    //}
    Ok(())
}
