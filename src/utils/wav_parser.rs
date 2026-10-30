use hound::WavReader;
use ndarray::Array2;
use spectrograms::{
    MelParams, 
    MelPowerSpectrogram,
    SpectrogramParams,
    StftParams,
    WindowType,
    nzu,
};
use anyhow::{Error, Ok, Result};

/// Parses wav data into one-dimentional Vec of f32(-1.0 to 1.0)
pub fn wav_to_raw_data(
    path: &str
) -> Result<Vec<f32>, Error>{
    let mut reader = WavReader::open(path)?;

    //Print out reader specification
    let spec = reader.spec();
    println!("Reader spec: {:?}", spec);

    //Collect data into a Vec
    let mut raw_data: Vec<f32> = Vec::new();
    for sample in reader.samples::<i16>(){
        let s = sample?;
        raw_data.push((s as f32)/(i16::MAX as f32));
    }
    Ok(raw_data)
}
