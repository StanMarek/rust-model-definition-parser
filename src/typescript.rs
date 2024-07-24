use crate::parser::{Field, Model};
use crate::utils::capitalize;
use std::collections::HashMap;

pub fn generate_typescript(models: HashMap<String, Model>) -> String {
    let mut ts = String::new();
    ts.push_str("export type WrappedValue<T> = { v?: T; a?: boolean; attributes?: unknown };\n\n");

    ts.push_str("export type Owner = {\n");
    ts.push_str("    id: WrappedValue<number>;\n");
    ts.push_str("    name: WrappedValue<string>;\n");
    ts.push_str("    effectiveEndYear: WrappedValue<number>;\n");
    ts.push_str("    years: OwnershipYears[];\n");
    ts.push_str("};\n\n");

    ts.push_str("export type OwnershipYears = {\n");
    ts.push_str("    percent: WrappedValue<number>;\n");
    ts.push_str("    year: WrappedValue<number>;\n");
    ts.push_str("};\n\n");

    ts.push_str("export type Operator = {\n");
    ts.push_str("    year: WrappedValue<number>;\n");
    ts.push_str("    id: WrappedValue<number>;\n");
    ts.push_str("    name: WrappedValue<string>;\n");
    ts.push_str("};\n\n");

    for (model_name, model) in &models {
        ts.push_str(&format!("export type {} = {{\n", capitalize(model_name)));

        if model_name == "asset" {
            add_asset_metadata_fields(&mut ts);
        }

        for field in &model.fields {
            let ts_type = get_typescript_type(field);
            ts.push_str(&format!("    {}: {};\n", field.name, ts_type));
        }

        ts.push_str("}\n\n");
    }

    ts
}

fn add_asset_metadata_fields(ts: &mut String) {
    ts.push_str("    _creator: WrappedValue<string>;\n");
    ts.push_str("    _created: WrappedValue<Date>;\n");
    ts.push_str("    _timestamp: WrappedValue<Date>;\n");
    ts.push_str("    _version: WrappedValue<number>;\n");
    ts.push_str("    operators: Operator[];\n");
    ts.push_str("    owners: Owner[];\n");
}

fn get_typescript_type(field: &Field) -> String {
    match field.field_type.as_str() {
        "text" => "WrappedValue<string>".to_string(),
        "number" => "WrappedValue<number>".to_string(),
        "list" | "vector" | "child" => {
            let sub_type = capitalize(field.sub_type.as_ref().unwrap());
            if field.field_type == "list" || field.field_type == "vector" {
                format!("{}[]", sub_type)
            } else {
                sub_type
            }
        }
        _ => "WrappedValue<any>".to_string(),
    }
}
