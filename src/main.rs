// SPDX-License-Identifier: MIT

use clap::Parser;
use std::{env, process};

mod config;
mod project;
mod qt;
mod workspace;

#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Add -o option to specify output file
    #[arg(short, long)]
    output_name: Option<String>,

    #[arg(short, long)]
    template_filename: Option<String>,

    #[command(flatten)]
    projects: CreateProjArgs,
}

#[derive(Debug, Clone, clap::Args)]
struct CreateProjArgs {
    #[arg(short, long)]
    create_project: Option<Option<String>>,

    #[arg(short = 'a', long)]
    create_template_project: Option<Option<String>>,
}

// suggestion is relative to cwd
fn suggest_output_filename(template_filename: &str) -> String {
    // get basename
    let template_filename = std::path::Path::new(template_filename)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    template_filename.to_string().replace(".template", "")
}

fn handle_projects_usecase() {
    if let Ok(args) = Args::try_parse() {
        if let Some(proj) = args.projects.create_project {
            if let Some(proj) = proj {
                let output_name = args.output_name.clone();

                process::exit(
                    match project::create_project_with_id(proj.as_str(), output_name) {
                        Ok(_) => 0,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            1
                        }
                    },
                );
            } else {
                process::exit(match project::print_projects() {
                    Ok(_) => 0,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        1
                    }
                });
            }
        }

        if let Some(proj) = args.projects.create_template_project {
            if let Some(proj) = proj {
                let output_name = args.output_name.clone();

                process::exit(
                    match project::create_template_project_with_id(proj.as_str(), output_name) {
                        Ok(_) => {
                            println!("Please edit project.json");
                            0
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            1
                        }
                    },
                );
            } else {
                process::exit(match project::print_projects() {
                    Ok(_) => 0,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        1
                    }
                });
            }
        }
    }
}

fn main() {
    // Handle -c, --create-project and -a, --create-template-project. Exits if handled.
    handle_projects_usecase();

    // Handle the main use case:

    let args = Args::parse();

    let config =
        config::Config::from_default_file().expect("Config file exists but can't be parsed");

    if let Err(e) = config.is_valid() {
        println!("Config: {}", e);
        process::exit(-1);
    }

    let template_filename = args
        .template_filename
        .clone()
        .expect("You're expected to pass: -t <template_filename>");

    let result: Result<(), workspace::Error> = if let Some(output_filename) = &args.output_name {
        // Case 1. User passed -o <output_filename>
        workspace::generate_from_file(
            template_filename,
            output_filename.to_string(),
            &config,
            env::consts::OS,
        )
    } else if config.has_output() {
        // Case 2. There's a .vscode-workspace-gen.json config file with either 'output_filename' or 'per_os_output_filename's set
        let targets = config.outputs().expect("Config has no usable targets");
        let mut last_result: Result<(), workspace::Error> = Ok(());
        for (os, output_filename) in targets {
            last_result = workspace::generate_from_file(
                template_filename.clone(),
                output_filename.clone(),
                &config,
                os,
            );

            if last_result.is_err() {
                break;
            }
        }

        last_result
    } else {
        // 3. Let's simply remove ".template" from the template filename
        let suggested_filename = suggest_output_filename(&args.template_filename.clone().unwrap());
        workspace::generate_from_file(
            template_filename,
            suggested_filename,
            &config,
            env::consts::OS,
        )
    };

    match result {
        Ok(_) => println!("File generated successfully"),
        Err(e) => {
            match e {
                workspace::Error::ExpectedRootObject => {
                    eprintln!("Error: Expected root object in JSON file");
                }
                workspace::Error::Io(e) => {
                    eprintln!("Error: {:?}", e);
                }
                workspace::Error::Json(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }

            process::exit(-1);
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use std::path::PathBuf;

    fn set_test_env() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/projects_folder");
        std::env::set_var("VSCODE_WORKSPACE_GEN_FOLDERS", d.to_str().unwrap());
    }

    #[test]
    fn test_create_template_project_cli() {
        set_test_env();

        // Test parsing of -a option
        let args = Args::try_parse_from(&["vscode-workspace-gen", "-a", "depends"]).unwrap();
        assert!(args.projects.create_template_project.is_some());
        assert_eq!(
            args.projects.create_template_project.unwrap().unwrap(),
            "depends"
        );

        // Test parsing of --create-template-project option
        let args =
            Args::try_parse_from(&["vscode-workspace-gen", "--create-template-project", "a"])
                .unwrap();
        assert!(args.projects.create_template_project.is_some());
        assert_eq!(args.projects.create_template_project.unwrap().unwrap(), "a");

        // Test that create_template_project_with_id works correctly
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_dir.push("cli_test_output");

        // Clean up if exists
        if test_dir.exists() {
            std::fs::remove_dir_all(&test_dir).unwrap();
        }

        // Test the function directly
        let result = project::create_template_project_with_id(
            "depends",
            Some("cli_test_output".to_string()),
        );
        assert!(result.is_ok());

        // Verify the output
        assert!(test_dir.exists());

        let project_json = test_dir.join("project.json");
        assert!(project_json.exists());

        let foo_txt = test_dir.join("foo.txt");
        assert!(foo_txt.exists());

        // Verify dependencies were NOT processed (no this.txt)
        let this_txt = test_dir.join("this.txt");
        assert!(!this_txt.exists());

        // Clean up
        std::fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_create_template_project_single_file_cli() {
        set_test_env();

        // Test single file project with -a option
        let args = Args::try_parse_from(&["vscode-workspace-gen", "-a", "a"]).unwrap();
        assert!(args.projects.create_template_project.is_some());

        // Use a dedicated test directory to avoid conflicts
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_dir = manifest_dir.join("cli_test_single_file");

        // Clean up if exists
        if test_dir.exists() {
            std::fs::remove_dir_all(&test_dir).unwrap();
        }

        // Create test directory
        std::fs::create_dir(&test_dir).unwrap();

        // Test the function with the test directory
        let result = project::create_template_project_with_id(
            "a",
            Some(test_dir.to_str().unwrap().to_string()),
        );
        assert!(result.is_ok());

        // Verify both files were created (including project.json)
        let this_txt = test_dir.join("this.txt");
        let project_json = test_dir.join("project.json");
        assert!(this_txt.exists());
        assert!(project_json.exists());

        // Clean up
        std::fs::remove_dir_all(test_dir).unwrap();
    }
}
