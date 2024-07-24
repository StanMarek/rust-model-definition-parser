use model_definition_parser_lib::{
    parser::{parse_model_definition, remove_duplicate_fields},
    typescript::generate_typescript,
};

fn main() {
    let mut models = parse_model_definition("demo/source/model.txt");
    remove_duplicate_fields(&mut models);
    let ts = generate_typescript(models);
    println!("{}", ts);
}
