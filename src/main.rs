use clap::{crate_version, Parser, Subcommand};

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
        #[arg(short = 'g', long = "generate")]
        generate: bool,
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
            // Implement search functionality here
        }
        VectaCommand::Index {
            directory,
            generate,
        } => {
            if generate {
                println!("Generating index for directory: {}", directory);
            } else {
                println!("Indexing directory: {}", directory);
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
