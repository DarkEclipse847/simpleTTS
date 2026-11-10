use std::path::Path;
use anyhow::{Error, Ok, Result};
use serde_jsonlines::json_lines;

use crate::utils::models;

pub fn parse_json(path: &Path) -> Result<Vec<models::ManifestItem>, Error>{
    let parsed_data = json_lines(path)?
        .map(|res| res.map_err(anyhow::Error::from))
        .collect::<Result<Vec<models::ManifestItem>>>()?;
    Ok(parsed_data)
}

