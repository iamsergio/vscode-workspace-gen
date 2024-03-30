// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};
use std::env;

const GEN_GLOBALS_KEY: &str = "gen.globals";
const GEN_DESCRIPTION_KEY: &str = "gen.description";
const GEN_OS_KEY: &str = "gen.os";

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
    discard_descriptions(&mut json[GEN_GLOBALS_KEY]);

    if let Some(globals) = json[GEN_GLOBALS_KEY].as_object().cloned() {
        json.as_object_mut().unwrap().remove(GEN_GLOBALS_KEY);
        replace_nesteds(&mut json, &globals)?;
    }

    remove_incompatible_os(&mut json);

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

// Argument is the key
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Nested(String),  // @{key}
    Inplace(String), // @@{key}
    None,
}

pub fn token_kind_from_str(s: &str) -> TokenKind {
    if s == "@{}" || s == "@@{}" {
        // We need a key name
        return TokenKind::None;
    }

    if s.starts_with("@{") && s.ends_with('}') {
        TokenKind::Nested(s[2..s.len() - 1].to_string())
    } else if s.starts_with("@@{") && s.ends_with('}') {
        TokenKind::Inplace(s[3..s.len() - 1].to_string())
    } else {
        TokenKind::None
    }
}

pub fn token_kind(value: &serde_json::Value) -> TokenKind {
    if value.is_string() {
        return token_kind_from_str(value.as_str().unwrap());
    }

    TokenKind::None
}

pub fn current_os() -> &'static str {
    env::consts::OS
}

/// If an object contains a "gen.os" key, it will only be included if the current OS is in the list
pub fn is_allowed_in_os(value: &serde_json::Value) -> bool {
    if let Some(obj) = value.as_object() {
        if let Some(os) = obj.get(GEN_OS_KEY) {
            return (os.is_array()
                && os
                    .as_array()
                    .unwrap()
                    .contains(&serde_json::Value::String(current_os().to_string())))
                || (os.is_string() && os.as_str().unwrap() == current_os());
        }
    }

    true
}

/// Replaces "${key}" instances
fn replace_nesteds(
    value: &mut serde_json::Value,
    globals: &serde_json::Map<String, serde_json::Value>,
) -> Result<(), Error> {
    if value.is_string() {
        // checks that the value conforms to the format @{contents}
        match token_kind(value) {
            TokenKind::Nested(key) => {
                if let Some(replacement_value) = globals.get(key.as_str()) {
                    *value = replacement_value.clone();
                } else {
                    println!("No replacement found for key: {}", key);
                }
            }
            TokenKind::Inplace(_) => (),
            TokenKind::None => (),
        }

        return Ok(());
    } else if value.is_array() {
        for v in value.as_array_mut().unwrap() {
            replace_nesteds(v, globals)?;
        }

        // expand $${key} instances
        let mut new_array_value = serde_json::Value::Array(vec![]);
        let new_array = new_array_value.as_array_mut().unwrap();
        for v in value.as_array().unwrap() {
            if let TokenKind::Inplace(key) = token_kind(v) {
                if let Some(replacement_value) = globals.get(key.as_str()) {
                    if replacement_value.is_array() {
                        for gv in replacement_value.as_array().unwrap() {
                            new_array.push(gv.clone());
                        }
                    } else {
                        new_array.push(replacement_value.clone());
                    }
                } else {
                    println!("No replacement found for key: {}", key);
                    new_array.push(v.clone());
                }
            } else {
                new_array.push(v.clone());
            }
        }

        *value = new_array_value;
    } else if value.is_object() {
        // Replace @{key}
        for (_, v) in value.as_object_mut().unwrap() {
            replace_nesteds(v, globals)?;
        }

        let mut new_object_value = serde_json::Value::Object(serde_json::Map::new());
        let new_object = new_object_value.as_object_mut().unwrap();

        // Replace @@{key}
        for (k, v) in value.as_object().unwrap() {
            if let TokenKind::Inplace(key) = token_kind_from_str(k.as_str()) {
                if let Some(replacement_value) = globals.get(key.as_str()) {
                    if !is_allowed_in_os(replacement_value) {
                        continue;
                    }

                    if replacement_value.is_object() {
                        for (rk, rv) in replacement_value.as_object().unwrap() {
                            // only insert if old object does not have the key
                            if !value.as_object().unwrap().contains_key(rk) {
                                new_object.insert(rk.clone(), rv.clone());
                            }
                        }
                    } else {
                        println!("Can only expand objects into objects");
                        new_object.insert(k.clone(), v.clone());
                    }
                } else {
                    println!("No replacement found for key: {}", key);
                    new_object.insert(k.clone(), v.clone());
                }
            } else {
                new_object.insert(k.clone(), v.clone());
            }
        }

        *value = new_object_value;
    }

    Ok(())
}

/// If an object has "gen.os", we remove it if the OS is not compatible
fn remove_incompatible_os(value: &mut serde_json::Value) {
    if value.is_object() {
        let value_obj = value.as_object_mut().unwrap();

        value_obj.retain(|_, v| is_allowed_in_os(v));

        value_obj.remove(GEN_OS_KEY);

        // obj.remove(GEN_OS_KEY);
        for (_, v) in value_obj {
            remove_incompatible_os(v);
        }
    } else if value.is_array() {
        let value_array = value.as_array_mut().unwrap();
        value_array.retain(is_allowed_in_os);

        for v in value_array {
            remove_incompatible_os(v);
        }
    }
}
