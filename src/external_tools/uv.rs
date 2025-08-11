use std::{path::PathBuf, process::Command};

use crate::external_tools::ExternalTool;

pub struct Uv {
    path: PathBuf,
}

impl ExternalTool for Uv {
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
        let tool = Uv::new_which("uv");
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }
}
