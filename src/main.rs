use clap::{Arg, Subcommand};
use model_definition_parser_lib::{
    parser::{parse_model_definition, remove_duplicate_fields},
    typescript::generate_typescript,
};

fn main() {
    let matches = App::new("Model Parser")
        .version("1.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Parses model definitions and generates TypeScript types")
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates TypeScript definitions from model file")
                .arg(
                    Arg::with_name("source")
                        .short("s")
                        .long("source")
                        .value_name("FILE")
                        .help("Sets the input model file")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("target")
                        .short("t")
                        .long("target")
                        .value_name("FILE")
                        .help("Sets the output TypeScript file")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("help").about("Displays this help message"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let source = matches.value_of("source").unwrap();
        let target = matches.value_of("target").unwrap();

        let mut models = parse_model_definition(source);
        dbg!(&models);
        remove_duplicate_fields(&mut models);
        let ts = generate_typescript(models);

        std::fs::write(target, ts).expect("Unable to write file");
        println!("TypeScript definitions generated at {}", target);
    } else if matches.subcommand_matches("help").is_some() {
        println!("{}", matches.usage());
    } else {
        println!("{}", matches.usage());
    }
}
