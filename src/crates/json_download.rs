use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct ResultItem { // public struct which has 2 public fields type: String
    pub tag: String,
    pub text: String,
}

pub fn save_to_json(results: &[ResultItem], filename: &str) -> Result<(), Box<dyn std::error::Error>> { // does stuff
    let json = serde_json::to_string_pretty(results)?; // makes it "pretty"

    let mut file = File::create(filename)?; // creates a new file with the specified filename
    file.write_all(json.as_bytes())?; // writes it to the json converted to bytes

    Ok(())
}