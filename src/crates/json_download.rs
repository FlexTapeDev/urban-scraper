use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct ResultItem {
    pub tag: String,
    pub text: String,
}

pub fn save_to_json(results: &[ResultItem], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(results)?;

    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}