use hound::WavReader;
use ndarray::{s, Array2};
use spectrograms::{LogParams, MelDbSpectrogram, MelParams, SpectrogramParams, StftParams, WindowType, nzu, };
use anyhow::{Error, Ok, Result, bail};
use non_empty_slice::{NonEmptySlice};
use std::{num::NonZeroUsize as NonZero, path::Path};

/// Parses wav data into one-dimentional Vec of f32(-1.0 to 1.0)
pub fn wav_to_raw_data(
    path: &Path
) -> Result<Vec<f64>, Error>{
    let mut reader = WavReader::open(path)?;

    //Print out reader specification
    let spec = reader.spec();
    println!("Reader spec: {:?}", spec);

    //Collect data into a Vec
    let mut raw_data: Vec<f64> = Vec::new();
    for sample in reader.samples::<i16>(){
        let s = sample?;
        raw_data.push((s as f64)/(i16::MAX as f64));
    }
    Ok(raw_data)
}

pub fn raw_data_to_spectrogram(
    raw_data: Vec<f64>,
    max_frames: usize
) -> Result<Array2<f64>, Error> {
    //let max_frames = max_frames.unwrap_or(500);

    let sample_rate = 24000.0;
    let stft_params = StftParams::new(nzu!(1024), nzu!(256), WindowType::Hanning, true)?;
    let spectrogram_params = SpectrogramParams::new(stft_params, sample_rate)?;
    let mel = MelParams::new(nzu!(80), 0.0, sample_rate/2.0)?;
    let log_params = LogParams::new(-80.0)?;

    let data = MelDbSpectrogram::compute(NonEmptySlice::try_from_slice(&raw_data).unwrap(), &spectrogram_params, &mel, Some(&log_params))?;
    let data_vec = data
        .data()
        .as_slice()
        .unwrap()
        .to_vec();
    let arr_2d = Array2::<f64>::from_shape_vec((data.n_frames().get(), data.n_bins().get()), data_vec)?;

    //println!("Frames: {:?}", data.n_frames());

    //Check if the audio is too large to use
    if data.n_frames() > NonZero::new(max_frames).unwrap() {
        bail!("Spectrogram has more frames than max_frames: {} > {}", data.n_frames(), max_frames);
        //eprintln!("Spectrogram has more frames than max_frames: {} > {}", data.n_frames(), max_frames)
    } else if data.n_frames() < NonZero::new(max_frames).unwrap() {
        let mut arr_2d_with_padding_empty = Array2::<f64>::from_elem((max_frames, data.n_bins().get()), -80.0);
        let mut arr_2d_with_padding = arr_2d_with_padding_empty.slice_mut(s![0..data.n_frames().get(), ..]);
        arr_2d_with_padding.assign(&arr_2d);
        Ok(arr_2d_with_padding_empty)
    } else {
        Ok(arr_2d)   
    }
}
