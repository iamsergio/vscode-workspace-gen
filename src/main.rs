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
    output_filename: Option<String>,

    #[arg(short, long)]
    template_filename: Option<String>,

    #[command(flatten)]
    projects: CreateProjArgs,
}

#[derive(Debug, Clone, clap::Args)]
struct CreateProjArgs {
    #[arg(short, long)]
    create_project: Option<Option<String>>,
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
                let output_filename = args.output_filename.clone();
                process::exit(
                    match project::create_project(proj.as_str(), output_filename) {
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
    }
}

fn main() {
    // Handle --create-project. Exits if handled.
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

    let result: Result<(), workspace::Error> = if let Some(output_filename) = &args.output_filename
    {
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
