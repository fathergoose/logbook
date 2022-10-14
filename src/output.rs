use textwrap::{dedent, wrap_columns};
use chrono::Local;
use crate::Cli;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const UL: &str = "\x1b[4m";

fn heading_text(cli: &Cli) -> String {
    if cli.tags {
        format!(
            "{}{}DateTime{}                 {}{}Text{}                                                    {}{}Tags{}",
            BOLD, UL, RESET, BOLD, UL, RESET, BOLD, UL, RESET
            )
    } else {
        format!(
            "{}{}DateTime{}                 {}{}Text{}",
            BOLD, UL, RESET, BOLD, UL, RESET
            )
    }

}

pub fn print_entries(entries: Vec<crate::data::Entry>, cli: &Cli) {
    println!("{}", heading_text(cli));
    for entry in entries {
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

