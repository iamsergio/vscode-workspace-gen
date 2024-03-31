// SPDX-License-Identifier: MIT

// Represents the contents of a .vscode-workspace-gen config file

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    json_indent: u32,

    output_filename: Option<String>,

    #[serde(default)]
    per_os_output_filenames: Option<PerOsOutputFileNames>,
}

/// Allows to generate output for different OSes. Can generate 3 files at once.
#[derive(Debug, Deserialize)]
pub struct PerOsOutputFileNames {
    windows: Option<String>,
    linux: Option<String>,
    macos: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            json_indent: 2,
            output_filename: None,
            per_os_output_filenames: None,
        }
    }
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        let conf: Config = serde_json::from_str(&contents)?;

        Ok(conf)
    }

    pub fn from_default_file() -> Result<Self, std::io::Error> {
        if std::path::Path::new(Config::filename()).exists() {
            Config::from_file(Config::filename())
        } else {
            Ok(Config::default())
        }
    }

    pub fn json_indent(&self) -> u32 {
        match self.json_indent {
            0..=10 => self.json_indent,
            _ => Config::default().json_indent,
        }
    }

    pub fn is_valid(&self) -> Result<(), String> {
        if self.output_filename.is_some() && self.per_os_output_filenames.is_some() {
            Err("Only one of output_filename or per_os_output_filenames can be set".to_string())
        } else {
            Ok(())
        }
    }

    pub fn has_target(&self) -> bool {
        self.output_filename.is_some() || self.per_os_output_filenames.is_some()
    }

    // returns a list of tuples with the OS and the output filename
    // else returns current OS
    pub fn targets(&self) -> Option<Vec<(&str, &String)>> {
        match &self.per_os_output_filenames {
            Some(per_os_output) => {
                let mut targets = Vec::new();
                if let Some(windows) = &per_os_output.windows {
                    targets.push(("windows", windows));
                }
                if let Some(linux) = &per_os_output.linux {
                    targets.push(("linux", linux));
                }
                if let Some(macos) = &per_os_output.macos {
                    targets.push(("macos", macos));
                }
                Some(targets)
            }
            None => self
                .output_filename
                .as_ref()
                .map(|output_filename| vec![(std::env::consts::OS, output_filename)]),
        }
    }

    fn filename() -> &'static str {
        ".vscode-workspace-gen.json"
    }
}
