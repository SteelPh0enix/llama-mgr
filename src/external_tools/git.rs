use std::{path::PathBuf, process::Command};

use crate::external_tools::ExternalTool;

pub struct Git {
    path: PathBuf,
}

impl ExternalTool for Git {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn global() -> Result<Self, which::Error>
    where
        Self: Sized,
    {
        which::which("git").map(Self::new)
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
        let tool = Git::global();
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }
}
