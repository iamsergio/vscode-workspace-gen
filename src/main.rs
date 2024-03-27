// SPDX-License-Identifier: MIT

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
    workspace.generate();
}
