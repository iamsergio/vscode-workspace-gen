// SPDX-License-Identifier: MIT

use std::process;

use clap::Parser;

mod workspace;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    template_filename: String,
}

fn suggest_target_filename(template_filename: &str) -> String {
    template_filename.to_string().replace(".template", "")
}

fn main() {
    let args = Args::parse();
    let target_filename = suggest_target_filename(&args.template_filename);

    match workspace::generate_from_file(args.template_filename, target_filename) {
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
