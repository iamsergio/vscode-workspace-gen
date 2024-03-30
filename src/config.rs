// SPDX-License-Identifier: MIT

// Represents the contents of a .vscode-workspace-gen config file

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    json_indent: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { json_indent: 2 }
    }
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        let conf: Config = serde_json::from_str(&contents)?;

        Ok(conf)
    }

    pub fn from_default_file() -> Result<Self, std::io::Error> {
        Config::from_file(Config::filename())
    }

    pub fn json_indent(&self) -> u32 {
        match self.json_indent {
            0..=10 => self.json_indent,
            _ => Config::default().json_indent,
        }
    }

    fn filename() -> &'static str {
        ".vscode-workspace-gen.json"
    }
}
