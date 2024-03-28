// SPDX-License-Identifier: MIT

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ExpectedRootObject,
}

pub struct Workspace {
    pub template_filename: String,
    pub target_filename: String,
}

impl Workspace {
    pub fn new(template_filename: String, target_filename: String) -> Self {
        Self {
            template_filename,
            target_filename,
        }
    }

    pub fn generate(&self) -> Result<(), Error> {
        // call generate with the contents of the file
        let template_contents =
            std::fs::read_to_string(&self.template_filename).map_err(|e| Error::IoError(e))?;

        let new_json = self.generate_from_string(&template_contents)?;

        // write json to target file
        let target_file =
            std::fs::File::create(&self.target_filename).map_err(|e| Error::IoError(e))?;

        serde_json::to_writer_pretty(target_file, &new_json).map_err(|e| Error::JsonError(e))?;

        Ok(())
    }

    pub fn generate_from_string(
        &self,
        template_contents: &String,
    ) -> Result<serde_json::Value, Error> {
        let mut json: serde_json::Value =
            serde_json::from_str(template_contents).map_err(|e| Error::JsonError(e))?;

        if !json.is_object() {
            return Err(Error::ExpectedRootObject);
        }

        let globals = json["globals"].as_object().cloned();
        json.as_object_mut().unwrap().remove("globals");

        self.replace_globals(&mut json, &globals)?;

        Ok(json)
    }

    /// On success, returns whether any globals were replaced (bool).
    fn replace_globals(
        &self,
        value: &mut serde_json::Value,
        globals: &Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<(), Error> {
        if value.is_string() {
            // checks that the value conforms to the format @{contents}
            let s = value.as_str().unwrap();
            if s.starts_with("@{") && s.ends_with("}") {
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
                self.replace_globals(v, globals)?;
            }
        } else if value.is_object() {
            for (_, v) in value.as_object_mut().unwrap() {
                self.replace_globals(v, globals)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_ctor() {
        let filename = "test";
        let workspace = Workspace::new(String::from("test"), String::from("test2"));
        assert_eq!(workspace.template_filename, filename);
    }

    #[test]
    fn test_unknown_file() {
        let workspace = Workspace::new(String::from("unknown.template"), String::from("unknown"));
        let result = workspace.generate();
        assert!(result.is_err());
        match result {
            Err(Error::IoError(e)) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => assert!(false, "Expected NotFound"),
            },
            _ => assert!(false, "Expected IoError"),
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

        let expected = r#"{
            "person": {
                "name": "John Doe",
                "age": 42
            }
        }"#;

        let template_filename = "test.template";
        let target_filename = "test";
        let workspace = Workspace::new(
            String::from(template_filename),
            String::from(target_filename),
        );
        let result = workspace
            .generate_from_string(&String::from(template))
            .unwrap();

        let expected_json: serde_json::Value = serde_json::from_str(expected).unwrap();

        assert_eq!(result, expected_json);
    }
}
