// SPDX-License-Identifier: MIT

// convenience to create project folders from a template
// Simply copies an entire folder or sub-folder from your VSCODE_WORKSPACE_GEN_FOLDERS
// Each folder should contain a project.json file with a description

use std::path::PathBuf;

use comfy_table::Table;

pub fn list_root_project_folder() -> Result<Vec<Project>, String> {
    let root_path = projects_root_path()?;

    if !root_path.exists() {
        return Err("Project does not exist".to_string());
    }

    if !root_path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let project_json_path = root_path.join("project.json");
    if project_json_path.exists() {
        return Err("root should not contain a project.json file".to_string());
    }

    list_folder(root_path.as_path())
}

fn projects_root_path() -> Result<std::path::PathBuf, String> {
    let path_str = std::env::var("VSCODE_WORKSPACE_GEN_FOLDERS").map_err(|e| e.to_string())?;

    let path = std::path::PathBuf::from(&path_str);
    Ok(path)
}

pub fn print_projects() -> Result<(), String> {
    let projects = list_root_project_folder()?;
    let mut table = Table::new();
    table.set_header(vec!["Description", "ID"]);

    for project in projects {
        table.add_row(vec![
            project.clone().description,
            project.project_id(projects_root_path()?.as_path()),
        ]);
    }
    println!("{table}");

    Ok(())
}

/// Describes the content of a project.json

#[derive(Clone)]
pub struct Project {
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

    /// The id is simply the path of the project.json file, without the prefix of the root folder
    fn project_id(self, root_path: &std::path::Path) -> String {
        let id = self.path.clone();
        // get path without the filename:
        let id = id.parent().unwrap();
        let id = id.strip_prefix(root_path).unwrap();

        String::from(id.to_str().unwrap())
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
