use std::{
    ffi::OsStr,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use crate::external_tools::ExternalTool;

pub struct CMake {
    path: PathBuf,
}

impl CMake {
    pub fn generate<T: AsRef<OsStr>>(
        &self,
        source_dir: T,
        build_dir: T,
        generator: T,
        additional_args: &[T],
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("-S")
            .arg(source_dir.as_ref())
            .arg("-B")
            .arg(build_dir.as_ref())
            .arg("-G")
            .arg(generator.as_ref());

        for arg in additional_args {
            command.arg(arg.as_ref());
        }

        command.status()
    }

    pub fn build<T: AsRef<OsStr>>(
        &self,
        build_dir: T,
        config: T,
        additional_args: &[T],
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("--build")
            .arg(build_dir.as_ref())
            .arg("--config")
            .arg(config.as_ref());

        for arg in additional_args {
            command.arg(arg.as_ref());
        }

        command.status()
    }

    pub fn install<T: AsRef<OsStr>>(
        &self,
        build_dir: T,
        config: T,
        additional_args: &[T],
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("--install")
            .arg(build_dir.as_ref())
            .arg("--config")
            .arg(config.as_ref());

        for arg in additional_args {
            command.arg(arg.as_ref());
        }

        command.status()
    }
}

impl ExternalTool for CMake {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn global() -> Result<Self, which::Error>
    where
        Self: Sized,
    {
        which::which("cmake").map(Self::new)
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
        let tool = CMake::global();
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }
}
