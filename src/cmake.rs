// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Write;

const CMAKE_PRESETS_JSON: &str = "CMakePresets.json";

pub fn generate_cmake_presets() -> Result<(), String> {
    if File::open(CMAKE_PRESETS_JSON).is_ok() {
        return Err(std::format!("{} already exists", CMAKE_PRESETS_JSON));
    }

    let mut file = File::create(CMAKE_PRESETS_JSON).map_err(|e| e.to_string())?;
    file.write_all(
        br#"{
  "version": 3,
  "configurePresets": [
    {
      "name": "dev",
      "description": "dev",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build-dev",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Debug"
      }
    },
    {
      "name": "rel",
      "description": "rel",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build-rel",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Release"
      }
    }
  ]
}"#,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
#[test]
fn test_generate_cmake_presets() {
    let _ = std::fs::remove_file(CMAKE_PRESETS_JSON);
    assert!(generate_cmake_presets().is_ok());
    assert!(File::open(CMAKE_PRESETS_JSON).is_ok());
    std::fs::remove_file(CMAKE_PRESETS_JSON).unwrap();
}
