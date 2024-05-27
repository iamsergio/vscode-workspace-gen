// SPDX-License-Identifier: MIT

// convenience to create project folders from a template
// Simply copies an entire folder or sub-folder from your VSCODE_WORKSPACE_GEN_FOLDERS
// Each folder should contain a project.json file with a description

use std::path::PathBuf;

pub fn list_root_project_folder() -> Result<Vec<Project>, String> {
    let root_path = std::env::var("VSCODE_WORKSPACE_GEN_FOLDERS").unwrap_or("".to_string());

    // check if folder_path is a directory and exists
    if root_path.is_empty() {
        return Err("No projects found".to_string());
    }

    let path = std::path::Path::new(&root_path);
    if !path.exists() {
        return Err("Project does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let project_json_path = path.join("project.json");
    if project_json_path.exists() {
        return Err("root should not contain a project.json file".to_string());
    }

    list_folder(path)
}

/// Describes the content of a project.json
struct Project {
    path: PathBuf,
    description: String,
}

impl Project {
    fn new(path: PathBuf, description: String) -> Self {
        Self { path, description }
    }

    fn from_file(project_json_path: PathBuf) -> Self {
        let contents = std::fs::read_to_string(&project_json_path).unwrap();

        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
        Self::new(
            project_json_path,
            json["description"].as_str().unwrap().to_string(),
        )
    }
}

fn list_folder(path: &std::path::Path) -> Result<Vec<Project>, String> {
    let mut result = Vec::new();
    let entries = std::fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let project_path = path.join("project.json");
            if project_path.exists() {
                result.push(Project::from_file(project_path));
            } else {
                if let Ok(sub_project) = list_folder(&path) {
                    result.extend(sub_project);
                } else {
                    return Err("Error reading sub-folder".to_string());
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_folder() {
        // get path of Cargo.toml
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/projects_folder");
        let result = list_folder(&d).unwrap();
        assert_eq!(result.len(), 3);

        // sort result based on description
        let mut result = result;
        result.sort_by(|a, b| a.description.cmp(&b.description));

        assert_eq!(result[0].description, "desc1");
        assert_eq!(result[1].description, "desc2");
        assert_eq!(result[2].description, "desc3");

        for r in result.iter() {
            assert_eq!(r.path.file_name().unwrap(), "project.json");
        }
    }
}
