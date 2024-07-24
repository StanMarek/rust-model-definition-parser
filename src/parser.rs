extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub sub_type: Option<String>,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

pub fn parse_model_definition(file_path: &str) -> HashMap<String, Model> {
    let mut models: HashMap<String, Model> = HashMap::new();
    let path = Path::new(file_path);
    let file = File::open(path).expect("Could not open file");

    let mut current_model: Option<Model> = None;

    let re_field = Regex::new(r"^\s+(\w+)\s+(\w+)(?:\s+type\s*:\s*(\w+))?").unwrap();

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("Could not read line").to_string();

        if should_skip_line(&line) {
            continue;
        }

        if line.starts_with("model") || line.starts_with("child") || line.starts_with("vector") {
            if let Some(model) = current_model {
                models.insert(model.name.clone(), model);
            }
            current_model = Some(Model {
                name: line.split_whitespace().nth(1).unwrap().to_string(),
                fields: vec![],
            });
        }

        if let Some(captures) = re_field.captures(&line) {
            if let Some(ref mut model) = current_model {
                model.fields.push(Field {
                    name: captures[1].to_string(),
                    field_type: captures[2].to_string(),
                    sub_type: captures.get(3).map(|m| m.as_str().to_string()),
                });
            }
        }

        if line.contains("$parent") {
            add_parent_specific_fields(&line, &mut current_model);
        }
    }

    if let Some(model) = current_model {
        models.insert(model.name.clone(), model);
    }

    models
}

fn should_skip_line(line: &str) -> bool {
    let skip_patterns = ["#", "model globals", "grouping"];
    if skip_patterns.iter().any(|&p| line.starts_with(p)) {
        return true;
    }

    let generic_asset_fields = ["owners", "operators"];
    if generic_asset_fields
        .iter()
        .any(|&field| line.trim_start().starts_with(field))
    {
        return true;
    }

    let omit_patterns = ["$if", "$min", "$max", "$textjoin", "$sum"];
    if omit_patterns.iter().any(|&pattern| line.contains(pattern)) && !line.contains("$parent") {
        return true;
    }

    false
}

fn add_parent_specific_fields(line: &str, current_model: &mut Option<Model>) {
    if let Some(ref mut model) = current_model {
        if line.trim_start().starts_with("year") {
            model.fields.push(Field {
                name: "year".to_string(),
                field_type: "text".to_string(),
                sub_type: None,
            });
        }
        if line.trim_start().starts_with("annual") {
            model.fields.push(Field {
                name: "annual".to_string(),
                field_type: "number".to_string(),
                sub_type: None,
            });
        }
    }
}

pub fn remove_duplicate_fields(models: &mut HashMap<String, Model>) {
    for model in models.values_mut() {
        let mut seen = HashSet::new();
        model.fields.retain(|field| seen.insert(field.name.clone()));
    }
}
