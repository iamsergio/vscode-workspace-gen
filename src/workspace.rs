// SPDX-License-Identifier: MIT

pub struct Workspace {
    pub filename: String,
}

impl Workspace {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    pub fn generate(&self) {
        println!("Generating workspace from file: {}", self.filename);
    }
}
