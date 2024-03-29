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

/// On success, returns whether any globals were replaced (bool).
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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_unknown_file() {
        let result = generate_from_file("unknown.template".to_string(), "unknown".to_string());
        assert!(result.is_err());
        match result {
            Err(Error::Io(e)) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => panic!("Expected NotFound"),
            },
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_string_replacements() {
        let template = r#"{
            "globals": {
                "name": "John Doe",
                "age": 42
            },
            "person": {
                "name": "@{name}",
                "age": "@{age}"
            }
        }"#;

        let expected: Value = serde_json::from_str(
            r#"{
            "person": {
                "name": "John Doe",
                "age": 42
            }
        }"#,
        )
        .unwrap();

        let result = generate_from_string(&String::from(template)).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_list_replacements() {
        let template = r#"{
            "globals": {
                "numbers": [1, 2, 3, 4, 5, 6, 7],
                "empty": []
            },
            "obj": {
                "l1": "@{numbers}",
                "l2": "@{empty}"
            }
        }"#;

        let expected: Value = serde_json::from_str(
            r#"{
            "obj": {
                "l1": [1, 2, 3, 4, 5, 6, 7],
                "l2": []
            }
        }"#,
        )
        .unwrap();

        let result = generate_from_string(&String::from(template)).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_obj_replacements() {
        let template = r#"{
            "globals": {
                "numbers": {
                    "one": 1,
                    "two": 2,
                    "three": 3
                },
                "empty": {}
            },
            "obj": {
                "l1": "@{numbers}",
                "l2": "@{empty}"
            }
        }"#;

        let expected: Value = serde_json::from_str(
            r#"{
            "obj": {
                "l1": {
                    "one": 1,
                    "two": 2,
                    "three": 3
                },
                "l2": {}
            }
        }"#,
        )
        .unwrap();

        let result = generate_from_string(&String::from(template)).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_obtest_gen_description() {
        let template = r#"{
            "globals": {
                "foo": {
                    "one": 1,
                    "gen.description": "This is a description"
                },
                "empty": {}
            },
            "obj": {
                "l1": "@{foo}"
            }
        }"#;

        let expected: Value = serde_json::from_str(
            r#"{
            "obj": {
                "l1": {
                    "one": 1
                }
            }
        }"#,
        )
        .unwrap();

        let result = generate_from_string(&String::from(template)).unwrap();
        assert_eq!(result, expected);
    }
}
