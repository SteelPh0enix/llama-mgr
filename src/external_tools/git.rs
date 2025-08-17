use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use crate::external_tools::ExternalTool;

pub struct Git {
    path: PathBuf,
}

impl Git {
    fn clone(
        &self,
        repo_path: impl AsRef<Path>,
        repo_url: impl AsRef<OsStr>,
        branch: Option<impl AsRef<OsStr>>,
    ) -> Result<(), ()> {
        let mut cmd = Command::new(&self.path);

        cmd.arg("clone");
        if branch.is_some() {
            cmd.arg("-b").arg(branch.unwrap());
        }
        cmd.arg(repo_url);
        cmd.arg(repo_path.as_ref().as_os_str());

        cmd.output().map(|_| ()).map_err(|_| ())
    }

    fn pull(&self, repo_path: impl AsRef<Path>) -> Result<(), ()> {
        let mut cmd = Command::new(&self.path);
        cmd.current_dir(repo_path);

        cmd.arg("pull");
        cmd.output().map(|_| ()).map_err(|_| ())
    }

    fn update_submodules(&self, repo_path: impl AsRef<Path>) -> Result<(), ()> {
        let mut cmd = Command::new(&self.path);
        cmd.current_dir(repo_path);

        cmd.arg("submodule")
            .arg("update")
            .arg("--init")
            .arg("--recursive")
            .arg("--remote");

        cmd.output().map(|_| ()).map_err(|_| ())
    }
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
