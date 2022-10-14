use clap::{ColorChoice, Parser};
use log::{debug, info};
pub mod data;
pub mod output;


#[derive(Parser, Debug)]
#[command(author("Alexander Ilseman"), version("0.1.3"), about("A moment happens once; what did you do with it?"), long_about = None)]
#[command(after_help = "Run without any arguments list last 10 entries.")]
#[command(color = ColorChoice::Auto)]
pub struct Cli {
    #[arg(short, long)]
    all: bool,

    #[clap(help = "Entry text. Omit to show last 10 entries")]
    entry: Vec<String>,

    #[arg(short, long)]
    number: Option<i32>,

    #[arg(short, long)]
    tags: bool,
}



fn list_last_entries(cli: &Cli) {
    let n = cli.number.unwrap_or(10);
    let entries = data::get_last_n_entries(n as usize).unwrap();
    output::print_entries(entries, cli);
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
        let entry = data::parse_entry(&cli.entry.join(" "));
        debug!("Entry: {:?}", entry);
        data::write_entry(&entry).unwrap();
        info!("Entry written");
    }
}
