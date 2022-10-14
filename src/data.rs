const FILENAME: &str = "/Users/al/.config/logbook/data.ndjson";
use chrono::{DateTime, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub date: DateTime<Utc>,
    pub text: String,
    pub tags: Vec<String>,
}

pub fn parse_entry(entry: &str) -> Entry {
    let date = Utc::now();
    let tags = entry
        .split_whitespace()
        .filter(|s| s.starts_with('#'))
        .map(|s| s.to_string())
        .collect();
    Entry {
        date,
        text: entry.to_string(),
        tags,
    }
}

pub fn write_entry(entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
    let entry_json = serde_json::to_string(entry)? + "\n";
    info!("Serialized Entry");
    debug!(": {}", entry_json);
    debug!("Writing entry to file");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(FILENAME)?;
    file.write_all(entry_json.as_bytes())?;
    Ok(())
}

pub fn get_last_n_entries(n: usize) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(FILENAME)?;
    let rev_lines = rev_lines::RevLines::new(std::io::BufReader::new(&file))?;
    let last_lines = rev_lines.take(n).map(|line| serde_json::from_str::<Entry>(&line)).collect::<Result<Vec<_>, _>>()?;
    let entries = last_lines.into_iter().rev().collect();
    Ok(entries)
}
