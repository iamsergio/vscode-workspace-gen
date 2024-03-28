// SPDX-License-Identifier: MIT

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

pub struct Workspace {
    pub filename: String,
}

impl Workspace {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    pub fn generate(&self) -> Result<(), Error> {
        println!("Generating workspace from file: {}", self.filename);

        // open the file
        let _file = std::fs::File::open(&self.filename).map_err(|e| Error::IoError(e))?;

        Ok(())
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

    #[test]
    fn test_unknown_file() {
        let workspace = Workspace::new(String::from("unknown"));
        let result = workspace.generate();
        assert!(result.is_err());
        match result {
            Err(Error::IoError(e)) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => assert!(false, "Expected NotFound"),
            },
            _ => assert!(false, "Expected IoError"),
        }
    }
}
