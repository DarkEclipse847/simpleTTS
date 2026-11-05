use hound::WavReader;
use ndarray::Array2;
use spectrograms::{LogParams, MelDbSpectrogram, MelParams, SpectrogramParams, StftParams, WindowType, nzu, };
use anyhow::{Error, Ok, Result};
use non_empty_slice::{NonEmptySlice};

/// Parses wav data into one-dimentional Vec of f32(-1.0 to 1.0)
pub fn wav_to_raw_data(
    path: &str
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
    raw_data: Vec<f64>
) -> Result<Array2<f64>, Error> {
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
    Ok(arr_2d)
}