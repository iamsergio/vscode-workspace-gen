// SPDX-License-Identifier: MIT

// convenience to create project folders from a template
// Simply copies an entire folder or sub-folder from your VSCODE_WORKSPACE_GEN_FOLDERS
// Each folder should contain a project.json file with a description

use std::path::PathBuf;

use comfy_table::Table;
use serde::Deserialize;

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
    table.set_header(vec!["Description", "ID", "Type"]);

    for project in projects {
        table.add_row(vec![
            project.clone().description,
            project.clone().project_id(projects_root_path()?.as_path()),
            project.type_str.unwrap_or("".to_string()),
        ]);
    }
    println!("{table}");

    Ok(())
}

/// Describes the content of a project.json

#[derive(Clone, Deserialize)]
pub struct Project {
    /// The path of the project.json
    #[serde(skip)]
    path: PathBuf,

    description: String,

    /// free form string to describe the type of project
    /// Just for display purposes
    #[serde(rename = "type")]
    type_str: Option<String>,

    /// List of other projects we depend on, such as .clang-format and other simple files
    depends: Option<Vec<String>>,
}

impl Project {
    fn new(path: PathBuf, json: Project) -> Self {
        Self { path, ..json }
    }

    fn from_file(project_json_path: PathBuf) -> Self {
        let contents = std::fs::read_to_string(&project_json_path).unwrap();

        let json: Project = serde_json::from_str(&contents).unwrap();
        Self::new(project_json_path, json)
    }

    /// The id is simply the path of the project.json file, without the prefix of the root folder
    fn project_id(self, root_path: &std::path::Path) -> String {
        let id = self.path.clone();
        // get path without the filename:
        let id = id.parent().unwrap();
        let id = id.strip_prefix(root_path).unwrap();

        String::from(id.to_str().unwrap())
    }

    fn base_folder(&self) -> Option<String> {
        let parent = self.path.parent().unwrap();
        parent
            .file_name()
            .map(|name| name.to_str().unwrap().to_string())
    }

    fn project_source_folder(&self) -> Result<PathBuf, String> {
        self.path
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or("Could not get parent folder".to_string())
    }

    fn is_single_file(&self) -> bool {
        self.type_str == Some("single_file".to_string())
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
            } else if let Ok(sub_project) = list_folder(&path) {
                result.extend(sub_project);
            } else {
                return Err("Error reading sub-folder".to_string());
            }
        }
    }

    Ok(result)
}

pub fn get_project(project_id: &str) -> Result<Project, String> {
    let root_path = projects_root_path()?;
    let project_path = root_path.join(project_id);

    if !project_path.exists() {
        return Err("Project does not exist".to_string());
    }

    if !project_path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let project_json_path = project_path.join("project.json");
    if !project_json_path.exists() {
        return Err("Project does not contain a project.json file".to_string());
    }

    Ok(Project::from_file(project_json_path))
}

fn absolute_path(output_dir: &str) -> PathBuf {
    let target_path = std::path::PathBuf::from(&output_dir);
    if target_path.is_absolute() {
        target_path
    } else {
        std::env::current_dir().unwrap().join(target_path)
    }
}

pub fn create_project_with_id(project_id: &str, output_dir: Option<String>) -> Result<(), String> {
    let project = get_project(project_id)?;
    create_project(project, output_dir, false)
}

pub fn create_template_project_with_id(
    project_id: &str,
    output_dir: Option<String>,
) -> Result<(), String> {
    let project = get_project(project_id)?;
    create_project(project, output_dir, true)
}

/// Creates a new folder with the project
pub fn create_project(
    project: Project,
    output_dir: Option<String>,
    creating_template: bool,
) -> Result<(), String> {
    if project.is_single_file() {
        return create_project_from_contents(project, output_dir, creating_template);
    }

    let absolute_target_path = if let Some(output_dir) = output_dir {
        absolute_path(output_dir.as_str())
    } else {
        std::env::current_dir().unwrap().join(
            project
                .base_folder()
                .ok_or("Could not get base folder".to_string())?,
        )
    };

    if !project.is_single_file() && absolute_target_path.exists() {
        return Err(std::format!(
            "Target path already exists {}",
            absolute_target_path.display()
        ));
    }

    copy_dir::copy_dir(project.path.parent().unwrap(), &absolute_target_path).unwrap();

    // Only process dependencies if not creating template
    if !creating_template {
        for dep in project.depends.unwrap_or_default() {
            let dep_proj = get_project(dep.as_str())?;
            let absolute_target_path_str = String::from(absolute_target_path.to_str().unwrap());
            create_project(dep_proj, Some(absolute_target_path_str), false)?;
        }
    }

    Ok(())
}

/// Creates the project but only copies the src contents, not the src directory itself
/// Useful to create a few files in the current directory
/// For now only copies files, not sub-directories, as it's not needed yet
fn create_project_from_contents(
    project: Project,
    output_dir: Option<String>,
    creating_template: bool,
) -> Result<(), String> {
    let absolute_target_path = if let Some(output_dir) = output_dir {
        absolute_path(output_dir.as_str())
    } else {
        std::env::current_dir().unwrap()
    };

    if !absolute_target_path.exists() {
        return Err(std::format!(
            "Target path doesn't exist {}",
            absolute_target_path.display()
        ));
    }

    let src_path = project.project_source_folder()?;

    // iterate src_path contents and copy files only
    for entry in std::fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Skip project.json unless creating template
        if !creating_template && entry.file_name() == "project.json" {
            continue;
        }

        if path.is_file() {
            let target_path = absolute_target_path.join(path.file_name().unwrap());
            std::fs::copy(&path, &target_path).unwrap();
        }
    }

    Ok(())
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
        assert_eq!(result.len(), 4);

        // sort result based on description
        let mut result = result;
        result.sort_by(|a, b| a.description.cmp(&b.description));

        assert_eq!(result[0].description, "Tests depends");
        assert_eq!(result[1].description, "desc1");
        assert_eq!(result[2].description, "desc2");
        assert_eq!(result[3].description, "desc3");

        for r in result.iter() {
            assert_eq!(r.path.file_name().unwrap(), "project.json");
        }
    }

    fn set_root_folder() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/projects_folder");

        // set env VSCODE_WORKSPACE_GEN_FOLDERS
        std::env::set_var("VSCODE_WORKSPACE_GEN_FOLDERS", d.to_str().unwrap());
    }

    #[test]
    fn test_get_project() {
        set_root_folder();
        let proj = get_project("c/d").unwrap();
        assert_eq!(proj.base_folder().unwrap(), "d");
    }

    #[test]
    fn test_create_project() {
        set_root_folder();

        create_project_with_id("c/d", None).unwrap();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("d");

        assert!(d.exists());
        std::fs::remove_dir_all(d).unwrap();

        create_project_with_id("c/d", Some("foo".to_string())).unwrap();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("foo");
        assert!(d.exists());
        std::fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn test_create_project_noparent() {
        set_root_folder();
        create_project_with_id("a", None).unwrap();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("a");

        assert!(!d.exists());
        d.pop();
        d.push("this.txt");
        assert!(d.exists());
        std::fs::remove_file(d).unwrap();
    }

    #[test]
    fn test_create_project_depends() {
        set_root_folder();
        create_project_with_id("depends", None).unwrap();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("depends");
        assert!(d.exists());

        d.push("foo.txt");
        assert!(d.exists());

        d.pop();
        d.push("this.txt");
        assert!(d.exists());
        d.pop();

        std::fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn test_create_template_project() {
        set_root_folder();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_template_depends");

        // Clean up if directory exists from previous test
        if d.exists() {
            std::fs::remove_dir_all(&d).unwrap();
        }

        create_template_project_with_id("depends", Some("test_template_depends".to_string()))
            .unwrap();
        assert!(d.exists());

        // Should have foo.txt from the project
        d.push("foo.txt");
        assert!(d.exists());

        d.pop();
        d.push("project.json");
        assert!(d.exists());

        // Should NOT have this.txt because dependencies are not processed
        d.pop();
        d.push("this.txt");
        assert!(!d.exists());

        d.pop();
        std::fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn test_create_template_project_single_file() {
        set_root_folder();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_template_single_file");

        // Clean up if directory exists from previous test
        if d.exists() {
            std::fs::remove_dir_all(&d).unwrap();
        }

        // Create test directory
        std::fs::create_dir(&d).unwrap();

        create_template_project_with_id("a", Some(d.to_str().unwrap().to_string())).unwrap();

        d.push("this.txt");
        assert!(d.exists());

        d.pop();
        d.push("project.json");
        assert!(d.exists());

        d.pop();
        std::fs::remove_dir_all(d).unwrap();
    }
}
