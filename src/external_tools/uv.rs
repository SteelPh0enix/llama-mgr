use crate::external_tools::ExternalTool;
use crate::external_tools::version::Version;

use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use which;

pub struct Uv {
    path: PathBuf,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PythonInstance {
    pub id: String,
    pub version: Version,
    pub path: Option<PathBuf>,
}

impl FromStr for PythonInstance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((id, path_info)) = s.split_once(" ") {
            let version: Version = id.parse().map_err(|_| ())?;
            let path = if path_info.contains("<download available>") {
                None
            } else if path_info.contains(" -> ") {
                if let Some((first_path, _)) = path_info.split_once(" -> ") {
                    // unwrap is infallible here
                    Some(PathBuf::from_str(first_path.trim()).unwrap())
                } else {
                    None
                }
            } else {
                // unwrap is infallible here
                Some(PathBuf::from_str(path_info.trim()).unwrap())
            };

            Ok(PythonInstance {
                id: String::from(id),
                version,
                path,
            })
        } else {
            Err(())
        }
    }
}

impl Uv {
    /// Returns a list of python instances returned from `uv`.
    /// Instances that have a path are currently installed.
    pub fn get_python_instances(&self) -> Result<Vec<PythonInstance>, std::io::Error> {
        let output = Command::new(&self.path)
            .arg("python")
            .arg("list")
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::other(format!(
                "'uv python list' failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let instances = stdout
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        Ok(instances)
    }

    /// Installs (or updates) selected python instance.
    pub fn install_python_instance(&self, instance: PythonInstance) -> Result<(), std::io::Error> {
        let mut command = Command::new(&self.path);
        command.arg("python").arg("install").arg(instance.id);

        let status = command.status()?;

        if !status.success() {
            return Err(std::io::Error::other(format!(
                "'uv python install' failed with status: {}",
                status
            )));
        }

        Ok(())
    }

    /// Installs (or updates) selected python instance by version.
    /// If multiple versions are available, picks the first one.
    /// If some parts of the version are missing, the latest one matching the missing part is installed.
    pub fn install_python_version(&self, version: Version) -> Result<(), std::io::Error> {
        let mut command = Command::new(&self.path);
        command
            .arg("python")
            .arg("install")
            .arg(version.to_string());

        let status = command.status()?;

        if !status.success() {
            return Err(std::io::Error::other(format!(
                "'uv python install' failed with status: {}",
                status
            )));
        }

        Ok(())
    }

    /// Uninstalls selected python instance.
    pub fn uninstall_python_instance(
        &self,
        instance: PythonInstance,
    ) -> Result<(), std::io::Error> {
        let mut command = Command::new(&self.path);
        command.arg("python").arg("uninstall").arg(instance.id);

        let status = command.status()?;

        if !status.success() {
            return Err(std::io::Error::other(format!(
                "'uv python uninstall' failed with status: {}",
                status
            )));
        }

        Ok(())
    }

    /// Uninstalls selected python instance by version.
    pub fn uninstall_python_version(&self, version: Version) -> Result<(), std::io::Error> {
        let mut command = Command::new(&self.path);
        command
            .arg("python")
            .arg("uninstall")
            .arg(version.to_string());

        let status = command.status()?;

        if !status.success() {
            return Err(std::io::Error::other(format!(
                "'uv python uninstall' failed with status: {}",
                status
            )));
        }

        Ok(())
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
        which::which("uv").map(Self::new)
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
        let python_instances = uv.get_python_instances();
        assert!(python_instances.is_ok());
        let instances = python_instances.unwrap();
        assert!(!instances.is_empty());
        for instance in instances {
            println!("{:?}", instance);
        }
    }
}
