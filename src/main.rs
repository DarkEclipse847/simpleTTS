use anyhow::{Ok, Result};

use crate::utils::wav_parser;

mod utils;
fn main() -> Result<()> {
    let parsed_data = wav_parser::wav_to_raw_data("train_data/crowd/0/00a0accd6e738b7b7feed2676217c74c.wav")?;
    for item in parsed_data.iter(){
        println!("{:?}", item)
    }
    Ok(())
}
