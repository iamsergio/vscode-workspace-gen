// SPDX-License-Identifier: MIT

use clap::Parser;
use std::{env, process};
mod workspace;

mod config;
#[cfg(test)]
mod tests;

#[cfg(feature = "qt")]
mod qt;

#[cfg(feature = "qt")]
const DEFAULT_WORKSPACE_FILE: &str = "vscode.code-workspace.template";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Add -o option to specify output file
    #[arg(short, long)]
    output_filename: Option<String>,

    #[arg(short, long)]
    template_filename: Option<String>,

    #[cfg(feature = "qt")]
    #[arg(long)]
    download_qtnatvis: bool,

    #[cfg(feature = "qt")]
    #[arg(long)]
    create_default_vscode_workspace: bool,
}

fn suggest_target_filename(template_filename: &str) -> String {
    template_filename.to_string().replace(".template", "")
}

#[cfg(feature = "qt")]
fn handle_qt_usecase() {
    if let Ok(args) = Args::try_parse() {
        if args.download_qtnatvis {
            process::exit(match qt::download_qtnatvis() {
                Ok(_) => {
                    println!("Downloaded qt6.natvis");
                    0
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    1
                }
            });
        } else if args.create_default_vscode_workspace {
            process::exit(
                match qt::generate_default_vscode_workspace(DEFAULT_WORKSPACE_FILE) {
                    Ok(_) => {
                        println!("Created vscode.code-workspace.template");
                        0
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        1
                    }
                },
            );
        }
    }
}

fn main() {
    // Handle --download-qtnatvis. Exits if handled.
    #[cfg(feature = "qt")]
    handle_qt_usecase();

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
        let target_filename = suggest_target_filename(&args.template_filename.clone().unwrap());
        workspace::generate_from_file(template_filename, target_filename, &config, env::consts::OS)
    };

    match result {
        Ok(_) => println!("Workspace generated successfully"),
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
