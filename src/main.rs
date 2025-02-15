use std::time::Instant;

use clap::{crate_version, Parser, Subcommand};
use vecta::indexer::{index_directory, read_index};

#[derive(Parser)]
#[command(name = "vecta")]
#[command(about = "A CLI tool for indexing and searching source code", long_about = None)]
struct VectaArgs {
    #[command(subcommand)]
    command: VectaCommand,
}

#[derive(Subcommand)]
enum VectaCommand {
    /// Search for a query in the indexed directories
    Search {
        /// Query to search for
        query: String,
    },
    /// Index a directory
    Index {
        /// Directory to index
        directory: String,

        /// Generate index instead of normal indexing
        #[arg(short = 'g', long = "global")]
        global: bool,
    },
    /// List indexed directories
    List,
    /// Remove an indexed directory
    Remove {
        /// Directory to remove (optional)
        directory: Option<String>,
    },
    /// Show version information
    Version,
}

fn main() {
    let args = VectaArgs::parse();

    match args.command {
        VectaCommand::Search { query } => {
            println!("Searching for: {}", query);
            let start_time = Instant::now();
            read_index(query);
            let elapsed_time = start_time.elapsed();
            println!("Search took: {:?}", elapsed_time);
        }
        VectaCommand::Index { directory, global } => {
            if global {
                println!("Generating global index for directory: {}", directory);
                index_directory(directory);
            } else {
                println!("Indexing directory: {}", directory);
                index_directory(directory);
            }
            // Implement indexing functionality here
        }
        VectaCommand::List => {
            println!("Listing indexed directories...");
            // Implement list functionality here
        }
        VectaCommand::Remove { directory } => {
            if let Some(dir) = directory {
                println!("Removing directory: {}", dir);
            } else {
                println!("Removing all indexed directories...");
            }
        }
        VectaCommand::Version => {
            println!("vecta {}", crate_version!());
        }
    }
}
