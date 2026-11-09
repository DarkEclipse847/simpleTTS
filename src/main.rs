use std::path::Path;


use anyhow::{Ok, Result};

use crate::utils::text_parser::parse_json;
use crate::utils::wav_parser;

mod utils;
fn main() -> Result<()> {
    let parsed_data = parse_json(Path::new("train_data/manifest.jsonl"))?;
    let parsed_data = wav_parser::wav_to_raw_data(Path::new("train_data/crowd/0/00a0accd6e738b7b7feed2676217c74c.wav"))?;
    let mel_data = wav_parser::raw_data_to_spectrogram(parsed_data, 500)?;
    
    for item in mel_data.iter(){
        println!("{:?}", item)
    }
    Ok(())
}
