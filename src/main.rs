// SPDX-License-Identifier: MIT

use clap::Parser;
use std::process;

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
    let target_filename = match &args.output_filename {
        Some(filename) => filename.clone(),
        _ => suggest_target_filename(&args.template_filename),
    };

    let config = config::Config::from_default_file().unwrap_or_default();

    match workspace::generate_from_file(args.template_filename, target_filename, &config) {
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
