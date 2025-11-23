use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConversionRecord {
    pub from: String,
    pub to: String,
    pub value: f64,
    pub result: f64,
    pub display_text: String,
}

pub const DEFAULT_FILE_PATH: &'static str = "conversion.json";

fn history_path() -> std::path::PathBuf {
    std::env::var("UNITCONV_HISTORY_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from(DEFAULT_FILE_PATH))
}

/// Loads conversion history from a JSON file
///
/// # Returns
///
/// A vector of ConversionRecord structs if successful, or an error if the file cannot be read.
pub fn load_history() -> Result<Vec<ConversionRecord>, std::io::Error> {
    let file_path = history_path();
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    // Try to read file as UTF-8 string; if encoding error, treat as empty history
    let data = match fs::read_to_string(&file_path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
            return Ok(Vec::new());
        }
        Err(e) => return Err(e),
    };
    let records: Vec<ConversionRecord> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
    Ok(records)
}

/// Saves a conversion record to the history file
///
/// # Arguments
///
/// * `record` - The ConversionRecord struct to be saved.
///
/// # Returns
///
/// Ok(()) if successful, or an error if the file cannot be written.
pub fn save_to_history(record: ConversionRecord) -> Result<(), std::io::Error> {
    let mut records: Vec<ConversionRecord> = load_history()?;
    records.push(record);
    let json_string = serde_json::to_string_pretty(&records)?;
    fs::write(history_path(), json_string)?;
    Ok(())
}
