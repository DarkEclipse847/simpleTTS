use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use anyhow::{Error, Ok, Result, bail};
use serde_jsonlines::json_lines;

use crate::utils::models::{self, ManifestItem};

pub fn parse_json(path: &Path) -> Result<Vec<models::ManifestItem>, Error>{
    let file = File::open(path)?;
    let parsed_data = json_lines(path)?
        .map(|res| res.map_err(anyhow::Error::from))
        .collect::<Result<Vec<models::ManifestItem>>>()?;
    Ok(parsed_data)
}
