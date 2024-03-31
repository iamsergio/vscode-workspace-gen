// SPDX-License-Identifier: MIT

use clap::Parser;
use std::{env, process};

mod workspace;

mod config;
#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Add -o option to specify output file
    #[arg(short, long)]
    output_filename: Option<String>,

    template_filename: String,
}

fn suggest_target_filename(template_filename: &str) -> String {
    template_filename.to_string().replace(".template", "")
}

fn main() {
    let args = Args::parse();
    let config =
        config::Config::from_default_file().expect("Config file exists but can't be parsed");

    if let Err(e) = config.is_valid() {
        println!("Config: {}", e);
        process::exit(-1);
    }

    let result: Result<(), workspace::Error> = if let Some(target_filename) = &args.output_filename
    {
        // Case 1. User passed -o <target_filename>
        workspace::generate_from_file(
            args.template_filename,
            target_filename.to_string(),
            &config,
            env::consts::OS,
        )
    } else if config.has_target() {
        // Case 2. There's a .vscode-workspace-gen.json config file with either 'output_filename' or 'per_os_output_filename's set
        let targets = config.targets().expect("Config has no usable targets");
        let mut last_result: Result<(), workspace::Error> = Ok(());
        for (os, target_filename) in targets {
            last_result = workspace::generate_from_file(
                args.template_filename.clone(),
                target_filename.clone(),
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
        let target_filename = suggest_target_filename(&args.template_filename);
        workspace::generate_from_file(
            args.template_filename,
            target_filename,
            &config,
            env::consts::OS,
        )
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
