use chrono::{DateTime, Local, Utc};
use clap::{ColorChoice, Parser};
use log::{debug, info};
use rev_lines::RevLines;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Write},
};
use textwrap::{dedent, wrap_columns};

const FILENAME: &str = "/Users/al/.config/logbook/data.ndjson";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const UL: &str = "\x1b[4m";

// TODO: Split up the reading and writing halves of the program into separate modules.
// TODO: Write tests - unit and integration.
// TODO: Add support for a config file, env vars, and/or command-line args.
// TODO: Add support for something like psql's `\e` command open in EDITOR
// TODO: Adjust print width based on terminal width
// TODO: Add support for printing "tags" field

#[derive(Parser, Debug)]
#[command(author("Alexander Ilseman"), version("0.1.3"), about("A moment happens once; what did you do with it?"), long_about = None)]
#[command(after_help = "Run without any arguments list last 10 entries.")]
#[command(color = ColorChoice::Auto)]
struct Cli {
    #[arg(short, long)]
    all: bool,

    #[clap(help = "Entry text. Omit to show last 10 entries")]
    entry: Vec<String>,

    #[arg(short, long)]
    number: Option<i32>,

    #[arg(short, long)]
    tags: bool,
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

fn heading_text(cli: &Cli) -> String {
    if cli.tags {
        format!(
            "{}{}DateTime{}                 {}{}Text{}                     {}{}Tags{}",
            BOLD, UL, RESET, BOLD, UL, RESET, BOLD, UL, RESET
            )
    } else {
        format!(
            "{}{}DateTime{}                 {}{}Text{}",
            BOLD, UL, RESET, BOLD, UL, RESET
            )
    }

}
fn list_last_entries(cli: &Cli) {
    let n = cli.number.unwrap_or(10);
    let file = File::open(FILENAME).unwrap();
    let rev_lines = RevLines::new(BufReader::new(&file)).unwrap();
    let last_lines = rev_lines.take(n as usize).collect::<Vec<_>>();
    // NOTE: For queies, we'll need a different approach to gathering the data
    println!("{}", heading_text(cli));
    for line in last_lines.iter().rev() {
        let entry = serde_json::from_str::<Entry>(&line).unwrap();
        let mut first_line = true;
        for line in format_entry(&entry.text) {
            if first_line {
                println!("");
                println!(
                    "{}{}{}{}{}",
                    BOLD,
                    entry
                        .date
                        .with_timezone(&Local)
                        .format("%a %h-%d %_I:%M %p"),
                    RESET,
                    "\t  ".to_owned() + &dedent(&line),
                    if cli.tags {
                        format!(" {}", entry.tags.join(" "))
                    } else {
                        "".to_owned()
                    }
                );
                first_line = false;
            } else {
                println!("{}", line);
            }
        }
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

fn format_entry(entry: &str) -> Vec<String> {
    #[cfg(feature = "hyphenation")]
    {
        use hyphenation::Load;
        let language = hyphenation::Language::EnglishUS;
        let dictionary = hyphenation::Standard::from_embedded(language).unwrap();
        options.word_splitter = WordSplitter::Hyphenation(dictionary);
    }
    wrap_columns(entry, 1 as usize, 50, "\t\t\t", "", "\t")
}

fn main() {
    let env = env_logger::Env::default().filter_or("RUST_ENV", "warn");
    env_logger::init_from_env(env);
    info!("Starting up");
    let cli = Cli::parse();
    info!("Parsed CLI Args");
    debug!("CLI: {:?}", cli);
    if cli.entry.is_empty() {
        list_last_entries(&cli);
    } else {
        let entry = parse_entry(&cli.entry.join(" "));
        debug!("Entry: {:?}", entry);
        write_entry(&entry).unwrap();
        info!("Entry written");
    }
}
