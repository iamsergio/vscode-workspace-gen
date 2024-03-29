// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

const GEN_DESCRIPTION_KEY: &str = "gen.description";

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    ExpectedRootObject,
}

pub fn generate_from_file(template_filename: String, target_filename: String) -> Result<(), Error> {
    // call generate with the contents of the file
    let template_contents = std::fs::read_to_string(template_filename).map_err(Error::Io)?;

    let new_json = generate_from_string(&template_contents)?;

    // write json to target file
    let target_file = std::fs::File::create(target_filename).map_err(Error::Io)?;
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut serializer = Serializer::with_formatter(target_file, formatter);
    new_json.serialize(&mut serializer).map_err(Error::Json)?;

    Ok(())
}

pub fn generate_from_string(template_contents: &str) -> Result<serde_json::Value, Error> {
    let mut json: serde_json::Value =
        serde_json::from_str(template_contents).map_err(Error::Json)?;

    if !json.is_object() {
        return Err(Error::ExpectedRootObject);
    }

    // Remove "gen.description" keys
    discard_descriptions(&mut json["globals"]);

    let globals = json["globals"].as_object().cloned();
    json.as_object_mut().unwrap().remove("globals");

    replace_globals(&mut json, &globals)?;

    Ok(json)
}

pub fn discard_descriptions(value: &mut serde_json::Value) {
    if value.is_object() {
        let obj = value.as_object_mut().unwrap();
        obj.remove(GEN_DESCRIPTION_KEY);
        for (_, v) in obj {
            discard_descriptions(v);
        }
    } else if value.is_array() {
        for v in value.as_array_mut().unwrap() {
            discard_descriptions(v);
        }
    }
}

fn replace_globals(
    value: &mut serde_json::Value,
    globals: &Option<serde_json::Map<String, serde_json::Value>>,
) -> Result<(), Error> {
    if value.is_string() {
        // checks that the value conforms to the format @{contents}
        let s = value.as_str().unwrap();
        if s.starts_with("@{") && s.ends_with('}') {
            let key = &s[2..s.len() - 1];
            if let Some(globals) = globals {
                if let Some(global_value) = globals.get(key) {
                    *value = global_value.clone();
                } else {
                    println!("No globals found for key: {}", key);
                }
            } else {
                println!("No globals found for key: {}", key);
            }
        }
        return Ok(());
    } else if value.is_array() {
        for v in value.as_array_mut().unwrap() {
            replace_globals(v, globals)?;
        }
    } else if value.is_object() {
        for (_, v) in value.as_object_mut().unwrap() {
            replace_globals(v, globals)?;
        }
    }

    Ok(())
}
