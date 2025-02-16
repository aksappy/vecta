use std::{
    fs::{create_dir, create_dir_all},
    io,
    path::PathBuf,
    time::Instant,
};
mod indexer;

use clap::{crate_version, Parser, Subcommand};
use indexer::{index_directory, read_index};

#[derive(Parser)]
#[command(name = "vecta")]
#[command(about = "A CLI tool for indexing and searching source code", long_about = None)]
struct VectaArgs {
    #[command(subcommand)]
    command: VectaCommand,
}

#[derive(Subcommand)]
enum VectaCommand {
    /// Initialize the Vecta environment
    Init {
        /// Directory to initialize Vecta in
        directory: String,
    },

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
    /// Destroy vecta directory
    Destroy {
        /// Directory to destroy
        directory: String,
    },
    /// Show version information
    Version,
}

fn main() {
    let required_directories = vec!["config", "data", "logs"];
    let args = VectaArgs::parse();

    match args.command {
        VectaCommand::Init { directory } => {
            println!("Initializing vecta in directory: {}", directory);
            println!(
                "This will create a .vecta directory and initialize the following directories:"
            );
            println!("- config");
            println!("- data");
            println!("- logs");
            println!("Continue? (Y/n)");
            let mut response = String::new();
            std::io::stdin()
                .read_line(&mut response)
                .expect("Failed to read user input, exiting...");

            if response.trim().to_lowercase() == "y" {
                for dir in &required_directories {
                    let result = create_dir_all(
                        PathBuf::from(&directory).join(".vecta").join(dir).as_path(),
                    );
                    if result.is_err() {
                        eprintln!("Error creating directory: {}", result.err().unwrap());
                        std::process::exit(1);
                    }
                }
            } else {
                println!("Initialization aborted.");
                std::process::exit(0);
            }
        }
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

        VectaCommand::Destroy { directory } => {
            println!("This will destroy the relevant vecta directories and all their contents.");
            println!("This is irreversible and will delete all data.");
            println!("Are you sure you want to proceed? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            if input.trim().to_lowercase() == "y" {
                let result =
                    std::fs::remove_dir_all(PathBuf::from(directory).join(".vecta").as_path());
                match result {
                    Ok(_) => println!("Vecta directory removed successfully."),
                    Err(e) => println!("Failed to remove directory: {}", e),
                }
            } else {
                println!("Operation cancelled.");
            }
        }
    }
}
