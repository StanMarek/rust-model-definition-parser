use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "Model Definition Parser",
    version = "1.1.0",
    about = "Parses model definitions and generates TypeScript types",
    long_about = "A comprehensive tool to parse model definitions from various sources and generate corresponding TypeScript types. Supports input from local files and Cosmos DB."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generates TypeScript definitions
    #[clap(subcommand)]
    Generate(GenerateArgs),
}

#[derive(Subcommand, Debug)]
pub enum GenerateArgs {
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

        /// Sets the Cosmos DB name to fetch configuration from
        #[arg(short, long)]
        db_name: String,

        /// Sets the Cosmos DB collection to fetch configuration from
        #[arg(short, long)]
        collection: String,

        /// Sets the search query
        /// db.collection.find({ search: "value" })
        #[arg(short, long)]
        search: String,

        /// Sets the model field to fetch
        #[arg(short, long)]
        value: String,

        /// Sets the model field to fetch
        /// Result of db.collection.find({ search: "value" }).project({ model_field: 1 })
        #[arg(short, long)]
        model_field: String,
    },
}
