use clap::Parser;

use vecta::indexer::{index_directory, merge_indices, read_index};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search string
    #[arg(short, long)]
    search: Option<String>,

    /// Index directory path. Defaults can be updated in config.
    #[arg(short, long)]
    dir: Option<String>,

    /// Config file path
    #[arg(short, long, default_value = "$HOME/.vecta/config")]
    config: Option<String>,

    /// Index a directory or file.
    #[arg(short, long, default_value = Option::None)]
    index: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let index = args.index.unwrap_or_else(|| false);
    let dir = args.dir.unwrap_or(String::from("."));
    let config = args
        .config
        .unwrap_or_else(|| String::from("$HOME/.vecta/config"));
    if index && !dir.is_empty() {
        index_directory(dir);
        merge_indices();
    }

    let search = args.search.unwrap_or_else(|| String::from(""));
    if !search.trim().is_empty() {
        // get time in milliseconds
        let start = std::time::Instant::now();
        println!("Searching for: {}", search);
        read_index(search);
        let duration = start.elapsed().as_millis();
        println!("Search took {} milliseconds", duration);
    }
}
