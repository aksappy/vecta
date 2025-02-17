use std::{ fs::{ create_dir_all, remove_dir_all }, io, path::PathBuf, time::Instant };

use clap::{ crate_version, Parser, Subcommand };
use vecta_lib::indexer::{ index_directory, read_index };

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

        /// Generate global index instead of normal indexing
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
    let args = VectaArgs::parse();

    match args.command {
        VectaCommand::Init { directory } => init(&directory),
        VectaCommand::Search { query } => search(&query),
        VectaCommand::Index { directory, global } => index(&directory, global),
        VectaCommand::List => list(),
        VectaCommand::Remove { directory } => remove(directory),
        VectaCommand::Version => version(),
        VectaCommand::Destroy { directory } => destroy(&directory),
    }
}

fn init(directory: &str) {
    println!("Initializing vecta in directory: {}", directory);
    println!("This will create a .vecta directory and initialize the following directories:");
    println!("- config");
    println!("- data");
    println!("- logs");
    println!("Continue? (Y/n)");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read user input");

    if response.trim().to_lowercase() == "y" {
        let required_directories = ["config", "data", "logs"];
        let vecta_dir = PathBuf::from(directory).join(".vecta");

        for dir in required_directories {
            let path = vecta_dir.join(dir);
            if !path.exists() {
                if let Err(e) = create_dir_all(&path) {
                    eprintln!("Error creating directory {}: {}", path.display(), e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!("Initialization aborted.");
        std::process::exit(0);
    }
}

fn search(query: &str) {
    println!("Searching for: {}", query);
    let start_time = Instant::now();
    read_index(String::from(query)); // Assumed to be defined in indexer module
    let elapsed_time = start_time.elapsed();
    println!("Search took: {:?}", elapsed_time);
}

fn index(directory: &str, global: bool) {
    let message = if global {
        "Generating global index for directory"
    } else {
        "Indexing directory"
    };
    println!("{}: {}", message, directory);
    index_directory(String::from(directory)); // Assumed to be defined in indexer module
}

fn list() {
    println!("Listing indexed directories...");
    // Implement list functionality here
}

fn remove(directory: Option<String>) {
    if let Some(dir) = directory {
        println!("Removing directory: {}", dir);
    } else {
        println!("Removing all indexed directories...");
    }
    // Implement remove functionality here
}

fn version() {
    println!("vecta {}", crate_version!());
}

fn destroy(directory: &str) {
    println!("This will destroy the relevant vecta directories and all their contents.");
    println!("This is irreversible and will delete all data.");
    println!("Are you sure you want to proceed? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if input.trim().to_lowercase() == "y" {
        let vecta_dir = PathBuf::from(directory).join(".vecta");
        match remove_dir_all(&vecta_dir) {
            Ok(_) => println!("Vecta directory removed successfully."),
            Err(e) => println!("Failed to remove directory {}: {}", vecta_dir.display(), e),
        }
    } else {
        println!("Operation cancelled.");
    }
}
