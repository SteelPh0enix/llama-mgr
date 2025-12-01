use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use crate::error::{Result, RuntimeError};
use crate::external_tools::ExternalTool;

pub struct Git {
    path: PathBuf,
}

impl Git {
    fn configure_git_command(&self, cmd: &mut Command) {
        // Configure git to not prompt for credentials
        cmd.env("GIT_TERMINAL_PROMPT", "0");
        cmd.env("GIT_CREDENTIAL_HELPER", "");
        cmd.env("GIT_TERMINAL_AUTHONLY", "0");
    }

    pub fn clone(
        &self,
        repo_path: impl AsRef<Path>,
        repo_url: impl AsRef<OsStr>,
        branch: Option<impl AsRef<OsStr>>,
    ) -> Result<()> {
        let mut cmd = Command::new(&self.path);

        self.configure_git_command(&mut cmd);

        cmd.arg("clone");
        if branch.is_some() {
            cmd.arg("-b").arg(branch.unwrap());
        }
        cmd.arg(repo_url);
        cmd.arg(repo_path.as_ref().as_os_str());

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RuntimeError::new(
                format!("git clone failed: {}", stderr),
                exitcode::DATAERR as u8,
            ));
        }

        Ok(())
    }

    pub fn pull(&self, repo_path: impl AsRef<Path>) -> Result<()> {
        let mut cmd = Command::new(&self.path);
        cmd.current_dir(repo_path);

        self.configure_git_command(&mut cmd);

        cmd.arg("pull");
        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RuntimeError::new(
                format!("git pull failed: {}", stderr),
                exitcode::DATAERR as u8,
            ));
        }

        Ok(())
    }

    pub fn update_submodules(&self, repo_path: impl AsRef<Path>) -> Result<()> {
        let mut cmd = Command::new(&self.path);
        cmd.current_dir(repo_path);

        self.configure_git_command(&mut cmd);

        cmd.arg("submodule")
            .arg("update")
            .arg("--init")
            .arg("--recursive")
            .arg("--remote");

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RuntimeError::new(
                format!("git submodule update failed: {}", stderr),
                exitcode::DATAERR as u8,
            ));
        }

        Ok(())
    }
}

impl ExternalTool for Git {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn global() -> std::result::Result<Self, which::Error>
    where
        Self: Sized,
    {
        which::which("git").map(|path| Self::new(path))
    }

    fn is_available(&self) -> bool {
        Command::new(&self.path).output().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RuntimeError;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_available() {
        let tool = Git::global();
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }

    #[test]
    #[serial]
    fn test_clone_repository() {
        let git = Git::global().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Clone the dummy repository
        let result = git.clone(
            repo_path,
            "https://github.com/SteelPh0enix/dummy_repo.git",
            None::<&str>,
        );

        assert!(result.is_ok(), "Failed to clone repository: {:?}", result);

        // Verify repository was cloned
        assert!(repo_path.exists());
        assert!(repo_path.join("README.md").exists());
        assert!(repo_path.join("LICENSE").exists());
        assert!(repo_path.join(".gitmodules").exists());
    }

    #[test]
    #[serial]
    fn test_clone_repository_with_branch() {
        let git = Git::global().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Clone with a specific branch (master should exist)
        let result = git.clone(
            repo_path,
            "https://github.com/SteelPh0enix/dummy_repo.git",
            Some("master"),
        );

        assert!(
            result.is_ok(),
            "Failed to clone repository with branch: {:?}",
            result
        );

        // Verify repository was cloned
        assert!(repo_path.exists());
        assert!(repo_path.join("README.md").exists());
    }

    #[test]
    #[serial]
    fn test_pull_repository() {
        let git = Git::global().unwrap();

        // First clone the repository
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        let clone_result = git.clone(
            repo_path,
            "https://github.com/SteelPh0enix/dummy_repo.git",
            None::<&str>,
        );
        assert!(
            clone_result.is_ok(),
            "Failed to clone repository for pull test"
        );

        // Test pull
        let pull_result = git.pull(repo_path);
        assert!(
            pull_result.is_ok(),
            "Failed to pull repository: {:?}",
            pull_result
        );

        // Verify repository still exists after pull
        assert!(repo_path.exists());
        assert!(repo_path.join("README.md").exists());
    }

    #[test]
    #[serial]
    fn test_update_submodules() {
        let git = Git::global().unwrap();

        // First clone the repository
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        let clone_result = git.clone(
            repo_path,
            "https://github.com/SteelPh0enix/dummy_repo.git",
            None::<&str>,
        );
        assert!(
            clone_result.is_ok(),
            "Failed to clone repository for submodule test"
        );

        // Test submodule update
        let submodule_result = git.update_submodules(repo_path);
        assert!(
            submodule_result.is_ok(),
            "Failed to update submodules: {:?}",
            submodule_result
        );

        // Verify submodule directory exists
        let submodule_path = repo_path.join("submodule/llama-mgr");
        assert!(submodule_path.exists(), "Submodule directory should exist");

        // Verify submodule content exists
        assert!(
            submodule_path.join("README.md").exists(),
            "Submodule README should exist"
        );
    }

    #[test]
    #[serial]
    fn test_clone_nonexistent_repository() {
        let git = Git::global().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Try to clone a non-existent repository
        let result = git.clone(
            repo_path,
            "https://github.com/SteelPh0enix/nonexistent_repo.git",
            None::<&str>,
        );

        // This should fail
        assert!(
            result.is_err(),
            "Should fail to clone non-existent repository"
        );
        if let Err(RuntimeError { message, .. }) = result {
            assert!(
                message.contains("git clone failed"),
                "Error message should indicate git clone failure"
            );
        }
    }

    #[test]
    #[serial]
    fn test_pull_non_git_directory() {
        let git = Git::global().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Create a non-git directory
        fs::write(
            repo_path.join("not_a_git_repo.txt"),
            "This is not a git repository",
        )
        .unwrap();

        // Try to pull from a non-git directory
        let result = git.pull(repo_path);

        // This should fail
        assert!(
            result.is_err(),
            "Should fail to pull from non-git directory"
        );
        if let Err(RuntimeError { message, .. }) = result {
            assert!(
                message.contains("git pull failed"),
                "Error message should indicate git pull failure"
            );
        }
    }
}
