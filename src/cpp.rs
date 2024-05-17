// SPDX-License-Identifier: MIT

use std::io::Write;

/// Creates a .clang-format file
pub fn generate_clang_format() -> Result<(), String> {
    let contents = include_bytes!("../templates/.clang-format");
    if contents.is_empty() {
        return Err(".clang-format is empty".to_string());
    }

    // Write the contents to the destination file
    let mut file = std::fs::File::create(".clang-format").map_err(|e| e.to_string())?;
    file.write_all(contents).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_clang_format() {
        assert!(generate_clang_format().is_ok());
        assert!(std::path::Path::new(".clang-format").exists());

        std::fs::remove_file(".clang-format").unwrap();
    }
}
