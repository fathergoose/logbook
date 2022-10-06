#![allow(unused_variables)]
use std::io::Write;

use chrono::{DateTime, Utc};
use clap::Parser;
use log::{debug, info};
use serde::{Deserialize, Serialize};

const FILENAME: &str = "/Users/al/.config/logbook/data.ndjson";

#[derive(Parser, Debug)]
#[command(author("Alexander Ilseman"), version("0.0.2"), about("A moment happens once; what did you do with it?"), long_about = None)]
#[command(after_help = "Run without any arguments list last 10 entries.")]
struct Cli {
    #[clap(help = "Entry text. Omit to show last 10 entries")]
    entry: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    date: DateTime<Utc>,
    text: String,
    tags: Vec<String>,
}

fn parse_entry(entry: &str) -> Entry {
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

fn list_last_entries(n: Option<i32>) {
    let file = std::fs::File::open(FILENAME).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut entries: Vec<Entry> = serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .map(|e| e.unwrap())
        .collect();
    entries.reverse();
    for entry in entries.iter().take(n.unwrap_or(10) as usize) {
        println!("{} {}", entry.date, entry.text);
    }
}

fn write_entry(entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() {
    let env = env_logger::Env::default().filter_or("RUST_ENV", "warn");
    env_logger::init_from_env(env);
    info!("Starting up");
    let cli = Cli::parse();
    info!("Parsed CLI Args");
    debug!("CLI: {:?}", cli);
    if cli.entry.is_empty() {
        list_last_entries(None);
    } else {
        let entry = parse_entry(&cli.entry.join(" "));
        debug!("Entry: {:?}", entry);
        write_entry(&entry).unwrap();
        info!("Entry written");
    }
}
