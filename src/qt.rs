// SPDX-License-Identifier: MIT

use std::io::Write;

/// Provides some extra convenience for Qt:
/// - Downloads the qt6.natvis file

const NATVIS_FILENAME: &str = "qt6.natvis";

pub fn download_qtnatvis() -> Result<(), String> {
    if std::path::Path::new(NATVIS_FILENAME).exists() {
        return Err(format!("{} already exists, bailing out", NATVIS_FILENAME));
    }

    let url =
        "https://raw.githubusercontent.com/KDABLabs/KDToolBox/master/qt/qt6_natvis/qt6.natvis";

    let mut response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Failed to download {}: {}", url, response.status()));
    }

    let mut file = std::fs::File::create(NATVIS_FILENAME).map_err(|e| e.to_string())?;
    response.copy_to(&mut file).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn generate_vscode_workspace(dst_filename: &str) -> Result<(), String> {
    // Read the file templates/qt.code-workspace.template using include_bytes
    let template_contents = include_bytes!("../templates/qt.code-workspace.template");
    if template_contents.is_empty() {
        return Err("Template is empty".to_string());
    }

    // Write the contents to the destination file
    let mut file = std::fs::File::create(dst_filename).map_err(|e| e.to_string())?;
    file.write_all(template_contents)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_qtnatvis() {
        let _ = std::fs::remove_file(NATVIS_FILENAME);

        let result = download_qtnatvis();
        if let Err(e) = &result {
            eprintln!("{}", e);
            panic!("Failed to download qt6.natvis");
        }

        std::fs::remove_file(NATVIS_FILENAME).unwrap();
    }

    #[test]
    fn test_create_default_workspace() {
        let dst_filename = "test.code-workspace";
        if std::path::Path::new(dst_filename).exists() {
            std::fs::remove_file(dst_filename).unwrap();
        }

        let result = generate_vscode_workspace(dst_filename);
        if let Err(e) = &result {
            eprintln!("{}", e);
            panic!("Failed to create vscode workspace");
        }

        std::fs::remove_file(dst_filename).unwrap();
    }
}
