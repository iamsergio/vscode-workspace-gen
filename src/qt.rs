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

pub fn generate_default_vscode_workspace(dst_filename: &str) -> Result<(), String> {
    // Read the file templates/qt.code-workspace.template using include_bytes
    let template_contents = include_bytes!("../templates/qt.code-workspace.template");
    if template_contents.is_empty() {
        return Err("Template is empty".to_string());
    }

    // Write the contents to the destination file
    let mut file = std::fs::File::create(dst_filename).map_err(|e| e.to_string())?;
    file.write_all(template_contents)
        .map_err(|e| e.to_string())?;

    println!("Don't forget to set env variable QT_SDK_INSTALL to the root of the Qt SDK, for example /home/user/Qt/
This folder should contain QtCreator, as it's required for LLDB pretty printers.");

    println!("\nDon't forget to set env variable QT_INSTALL to the specific Qt installed version, for example/opt/Qt/6.2.0/gcc_64
This is required for debugger source mapping\n");

    Ok(())
}

pub fn suggest_needed_env_vars(template_contents: &str) {
    // create a string to string map, where key is env name and value is the message
    let env_vars = vec![
        (
            "QT_SDK_INSTALL",
            "the root of the Qt SDK, for example /home/user/Qt/",
        ),
        (
            "QT_INSTALL",
            "the specific Qt installed version, for example /opt/Qt/6.2.0/gcc_64",
        ),
    ];

    // iterate over the map and check if the env var exists
    for (varname, message) in env_vars {
        if template_contents.contains(std::format("$${:}", varname))
            && std::env::var(varname).is_err()
        {
            println!(
                "Env variable {} isn't set! Should be set to {}",
                varname, message
            );
        }
    }
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

        let result = generate_default_vscode_workspace(dst_filename);
        if let Err(e) = &result {
            eprintln!("{}", e);
            panic!("Failed to create vscode workspace");
        }

        std::fs::remove_file(dst_filename).unwrap();
    }
}
