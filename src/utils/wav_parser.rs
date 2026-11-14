use hound::WavReader;
use ndarray::{s, Array2};
use spectrograms::{LogParams, MelDbSpectrogram, MelParams, SpectrogramParams, StftParams, WindowType, nzu, };
use anyhow::{Error, Ok, Result};
use non_empty_slice::{NonEmptySlice};
use std::path::Path;
use std::num::NonZero;

/// Parses wav data into one-dimentional Vec of f32(-1.0 to 1.0)
pub fn wav_to_raw_data(
    path: &Path
) -> Result<(Vec<f64>, u32), Error>{
    let mut reader = WavReader::open(path)?;

    //Print out reader specification
    let spec = reader.spec();
    //println!("Reader spec: {:?}", spec);

    //Collect data into a Vec
    let mut raw_data: Vec<f64> = Vec::new();
    for sample in reader.samples::<i16>(){
        let s = sample?;
        raw_data.push((s as f64)/(i16::MAX as f64));
    }
    Ok((raw_data, spec.sample_rate))
}

pub fn raw_data_to_spectrogram(
    raw_data: Vec<f64>,
    sample_rate: u32,
    max_frames: usize
) -> Result<(Array2<f64>, usize), Error> {
    // let sample_rate = 24000.0;
    
    let stft_params = StftParams::new(nzu!(1024), nzu!(256), WindowType::Hanning, true)?;
    let spectrogram_params = SpectrogramParams::new(stft_params, sample_rate as f64)?;
    let mel = MelParams::new(nzu!(80), 0.0, (sample_rate as f64)/2.0)?;
    let log_params = LogParams::new(-80.0)?;

    let data = MelDbSpectrogram::compute(NonEmptySlice::try_from_slice(&raw_data).unwrap(), &spectrogram_params, &mel, Some(&log_params))?;
    let data_vec = data
        .data()
        .as_slice()
        .unwrap()
        .to_vec();
    let arr_2d = Array2::<f64>::from_shape_vec((data.n_bins().get(), data.n_frames().get()), data_vec)?;
    if data.n_frames() < NonZero::new(max_frames).unwrap() {
        let mut arr_2d_with_padding_empty = Array2::<f64>::from_elem((max_frames, data.n_bins().get()), -80.0);
        let mut arr_2d_with_padding = arr_2d_with_padding_empty.slice_mut(s![0..data.n_frames().get(), ..]);
        arr_2d_with_padding.assign(&arr_2d);
        Ok((arr_2d_with_padding_empty, data.n_frames().get()))
    } else {
        //Can we pass more than max_frames?
        Ok((arr_2d, data.n_frames().get()))
    }
}

pub fn wav_duration(path: &Path) -> Result<f32, Error>{
    let reader = WavReader::open(path)?;
    let spec = reader.spec();
    let duration = reader.duration()as f32 / spec.sample_rate as f32;
    Ok(duration)
}