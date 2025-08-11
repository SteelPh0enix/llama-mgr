use std::{path::PathBuf, process::Command};

use crate::external_tools::ExternalTool;
use which;

pub struct Uv {
    path: PathBuf,
}

pub struct PythonInstance {
    version: String,
    path: PathBuf,
}

impl Uv {
    pub fn list_python_instances(&self) -> Result<Vec<PythonInstance>, std::io::Error> {
        let output = Command::new(&self.path)
            .arg("python")
            .arg("list")
            .output()?;

        println!("{:?}", output);
        Ok(vec![])
    }
}

impl ExternalTool for Uv {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn global() -> Result<Self, which::Error>
    where
        Self: Sized,
    {
        which::which("uv").map(|path| Self::new(path))
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
        let uv = Uv::global();
        assert!(uv.is_ok());
        let uv = uv.unwrap();
        assert!(uv.is_available());
    }

    #[test]
    fn test_list_python_instances() {
        let uv = Uv::global().unwrap();
        let python_instances = uv.list_python_instances();
    }
}
