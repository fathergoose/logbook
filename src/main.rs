use std::{io::{Write, BufReader}, fs::File};

use chrono::{DateTime, Local, TimeZone, Utc};
use clap::{ColorChoice, Parser};
use log::{debug, info};
use rev_lines::RevLines;
use serde::{Deserialize, Serialize};

const FILENAME: &str = "/Users/al/.config/logbook/data.ndjson";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const UL: &str = "\x1b[4m";

// TODO: Add support for text wrapping via `textwrap` crate
// TODO: Potentially fixed-width columns via the `tabwriter` crate.
// TODO: Split up the reading and writing halves of the program into separate modules.
// TODO: Write tests - unit and integration.
// TODO: Add support for a config file, env vars, and/or command-line args.

#[derive(Parser, Debug)]
#[command(author("Alexander Ilseman"), version("0.0.2"), about("A moment happens once; what did you do with it?"), long_about = None)]
#[command(after_help = "Run without any arguments list last 10 entries.")]
#[command(color = ColorChoice::Auto)]
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
    let n = n.unwrap_or(10);
    let file = File::open(FILENAME).unwrap();
    let rev_lines = RevLines::new(BufReader::new(&file)).unwrap();
    let last_lines = rev_lines.take(n as usize).collect::<Vec<_>>();
    // TODO: For queies, we'll need a different approach to gathering the data
    println!("{}Last {} entries:{}\n", BOLD, n, RESET);
    println!(
        "{}{}DateTime{}                     {}{}Text{}",
        BOLD, UL, RESET, BOLD, UL, RESET
    );
    for line in last_lines.iter().rev() {
        let entry = serde_json::from_str::<Entry>(&line).unwrap();
        println!(
            "{}{}{} {}",
            BOLD,
            entry.date.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S"),
            RESET,
            entry.text,
        );
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
