use std::{fs, path::Path};

use clap::Parser;
use model_definition_parser::{
    cli::{Args, Commands, GenerateArgs},
    parser::{parse_model_definition, remove_duplicate_fields},
    typescript::generate_typescript,
};
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[tokio::main]
async fn main() {
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
            GenerateArgs::Cosmos {
                url,
                db_name,
                collection,
                search,
                value,
                model_field,
            } => {
                // Placeholder for Cosmos DB integration logic
                println!(
                    "Fetching data from Cosmos DB at {} with database {} and collection {} using search query {} and model field {}",
                    url, db_name, collection, search, model_field
                );

                let client = Client::with_uri_str(url).await.unwrap();
                let db = client.database(db_name);

                let collection: Collection<Document> = db.collection(collection);

                let query = doc! { search: value };

                let document = collection
                    .find_one(query)
                    .await
                    .unwrap()
                    .expect("Failed to execute find_one");

                let projected_field = document
                    .get(model_field)
                    .expect("Field not found in the document");

                let output_path = "output.txt";
                if let Some(parent) = Path::new(output_path).parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).expect("Unable to create directories");
                    }
                }

                // Serialize the field and write to the file
                let serialized = serde_json::to_string_pretty(projected_field)
                    .unwrap()
                    .split('\n')
                    .collect::<String>();

                for line in serialized.lines() {
                    println!("{}", line);
                }

                // println!("Field: {}", serialized);

                // fs::write(output_path, serialized).expect("Unable to write file");

                // println!("Document: {:?}", document);
            }
        },
    }
}
