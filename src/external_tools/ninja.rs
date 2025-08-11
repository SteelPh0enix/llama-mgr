use std::{path::PathBuf, process::Command};

use crate::external_tools::ExternalTool;

pub struct Ninja {
    path: PathBuf,
}

impl ExternalTool for Ninja {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn is_available(&self) -> bool {
        Command::new(&self.path).output().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_available() {
        let tool = Ninja::new_which("ninja");
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }
}
