use std::{fs, path::Path};

use clap::{arg, command, Parser, Subcommand};
use model_definition_parser::{
    parser::{parse_model_definition, remove_duplicate_fields},
    typescript::generate_typescript,
};

#[derive(Parser, Debug)]
#[command(
    name = "Model Definition Parser",
    version = "1.1.0",
    about = "Parses model definitions and generates TypeScript types",
    long_about = "A comprehensive tool to parse model definitions from various sources and generate corresponding TypeScript types. Supports input from local files and Cosmos DB."
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates TypeScript definitions
    #[clap(subcommand)]
    Generate(GenerateArgs),
}

#[derive(Subcommand, Debug)]
enum GenerateArgs {
    /// Generates TypeScript definitions from model file
    File {
        /// Sets the input model file
        #[arg(short, long)]
        source: String,

        /// Sets the output TypeScript file
        #[arg(short, long)]
        target: String,
    },
    /// Generates TypeScript definitions from Cosmos DB
    Cosmos {
        /// Sets the Cosmos DB URL
        #[arg(short, long)]
        url: String,

        /// Sets the asset group - collection name
        #[arg(short, long)]
        asset_group: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Generate(generate_command) => match generate_command {
            GenerateArgs::File { source, target } => {
                let mut models = parse_model_definition(source);

                remove_duplicate_fields(&mut models);
                let ts = generate_typescript(models);

                if let Some(parent) = Path::new(target).parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).expect("Unable to create directories");
                    }
                }

                fs::write(target, ts).expect("Unable to write file");
                println!("TypeScript definitions generated at {}", target);
            }
            GenerateArgs::Cosmos { url, asset_group } => {
                // Placeholder for Cosmos DB integration logic
                println!(
                    "Fetching data from Cosmos DB at {} for asset group {}",
                    url, asset_group
                );
            }
        },
    }
}
