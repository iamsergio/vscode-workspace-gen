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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_ctor() {
        let filename = "test";
        let workspace = Workspace::new(String::from(filename));
        assert_eq!(workspace.filename, filename);
    }
}
