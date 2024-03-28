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

fn main() {
    let args = Args::parse();

    let workspace = Workspace::new(args.filename);
    match workspace.generate() {
        Ok(_) => println!("Workspace generated successfully"),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}
