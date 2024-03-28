// SPDX-License-Identifier: MIT

use std::process;

use clap::Parser;

mod workspace;

use crate::workspace::Workspace;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

fn suggest_target_filename(template_filename: &str) -> String {
    template_filename.to_string().replace(".template", "")
}

fn main() {
    let args = Args::parse();

    let target_filename = suggest_target_filename(&args.filename);
    let workspace = Workspace::new(args.filename, target_filename);
    match workspace.generate() {
        Ok(_) => println!("Workspace generated successfully"),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}
