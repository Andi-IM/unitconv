use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversionRecord {
    pub from: String,
    pub to: String,
    pub value: f64,
    pub result: f64,
    pub display_text: String,
}

impl ConversionRecord {
    const FILE_PATH: &'static str = "conversion.json";

    pub fn load_history() -> Result<Vec<ConversionRecord>, std::io::Error> {
        if !Path::new(Self::FILE_PATH).exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(Self::FILE_PATH)?;
        let records: Vec<ConversionRecord> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
        Ok(records)
    }

    pub fn save_to_history(record: ConversionRecord) -> Result<(), std::io::Error> {
        let mut records = Self::load_history()?;
        records.push(record);
        let json_string = serde_json::to_string_pretty(&records)?;
        fs::write(Self::FILE_PATH, json_string)?;
        Ok(())
    }
}
