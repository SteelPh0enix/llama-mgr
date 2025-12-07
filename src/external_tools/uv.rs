use crate::external_tools::ExternalTool;
use crate::external_tools::version::Version;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
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

#[derive(Debug)]
pub struct VirtualEnvironment {
    pub path: PathBuf,
    pub python_instance: PythonInstance,
}

#[derive(Debug)]
pub struct VirtualEnvironmentShell<'a> {
    pub venv: &'a VirtualEnvironment,
    pub shell_process: Child,
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
                    Some(PathBuf::from_str(first_path.trim()).unwrap())
                } else {
                    None
                }
            } else {
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

pub type UvResult<T> = Result<T, std::io::Error>;

impl Uv {
    /// Returns a list of python instances returned from `uv`.
    /// Instances that have a path are currently installed.
    pub fn get_python_instances(&self) -> UvResult<Vec<PythonInstance>> {
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
    pub fn install_python_instance(&self, instance: PythonInstance) -> UvResult<()> {
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
    pub fn install_python_version(&self, version: Version) -> UvResult<()> {
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
    pub fn uninstall_python_instance(&self, instance: PythonInstance) -> UvResult<()> {
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
    pub fn uninstall_python_version(&self, version: Version) -> UvResult<()> {
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

    /// Creates a virtual environment using a specific Python version
    pub fn create_venv<T: AsRef<Path>>(
        &self,
        path: T,
        version: Version,
    ) -> UvResult<VirtualEnvironment> {
        let mut command = Command::new(&self.path);
        command
            .arg("venv")
            .arg("--python")
            .arg(version.to_string())
            .arg(path.as_ref());

        let status = command.status()?;

        if !status.success() {
            return Err(std::io::Error::other(format!(
                "'uv venv' failed with status: {}",
                status
            )));
        }

        // Get the python instance that was used to create the venv
        let python_instances = self.get_python_instances()?;

        // Find the instance that matches our version
        let python_instance = python_instances
            .into_iter()
            .find(|instance| instance.version == version)
            .ok_or_else(|| {
                std::io::Error::other("Failed to find Python instance for virtual environment")
            })?;

        Ok(VirtualEnvironment {
            path: path.as_ref().to_path_buf(),
            python_instance,
        })
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

impl VirtualEnvironment {
    /// Creates a new `bash` shell, activates the virtual environment inside it, and then returns it.
    pub fn create_shell(&self) -> UvResult<VirtualEnvironmentShell<'_>> {
        let activate_script_path = self.path.join("bin").join("activate");
        let command = format!("source {} && exec bash", activate_script_path.display());

        let shell_process = Command::new("bash").arg("-c").arg(&command).spawn()?;

        Ok(VirtualEnvironmentShell {
            venv: &self,
            shell_process,
        })
    }
}

impl VirtualEnvironmentShell<'_> {
    /// Runs a python script inside the virtual environment
    pub fn run_python_script<T: AsRef<Path>>(&self, script_path: T) -> UvResult<()> {
        todo!()
    }

    /// Installs Python packages inside the virtual environment
    pub fn install_packages<T: Into<String>>(&self, packages_list: &[T]) -> UvResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

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

    #[test]
    #[serial]
    fn test_install_uninstall_python_instance() {
        let uv = Uv::global().unwrap();

        // Find the full ID for python 3.8.20
        let python_instances = uv.get_python_instances().unwrap();
        let instance_template = python_instances
            .into_iter()
            .find(|i| i.id.starts_with("cpython-3.8.20"))
            .expect("Python 3.8.20 not found in uv list");

        let instance_id = instance_template.id.clone();

        // Ensure it's not installed before the test
        if let Some(instance_to_uninstall) = uv
            .get_python_instances()
            .unwrap()
            .into_iter()
            .find(|i| i.id == instance_id && i.path.is_some())
        {
            uv.uninstall_python_instance(instance_to_uninstall).unwrap();
        }

        // Get the instance to install (it should have path: None)
        let instance_to_install = uv
            .get_python_instances()
            .unwrap()
            .into_iter()
            .find(|i| i.id == instance_id)
            .unwrap();
        assert!(instance_to_install.path.is_none());

        // Install
        uv.install_python_instance(instance_to_install).unwrap();

        // Verify installation
        let installed_instance = uv
            .get_python_instances()
            .unwrap()
            .into_iter()
            .find(|i| i.id == instance_id && i.path.is_some())
            .expect("Instance not found after install");
        assert!(installed_instance.path.is_some());

        // Uninstall
        uv.uninstall_python_instance(installed_instance).unwrap();

        // Verify uninstallation
        let instances_after_uninstall = uv.get_python_instances().unwrap();
        assert!(
            instances_after_uninstall
                .iter()
                .find(|i| i.id == instance_id && i.path.is_some())
                .is_none(),
            "Instance should not have a path after uninstall"
        );
    }

    #[test]
    #[serial]
    fn test_install_uninstall_python_version() {
        let uv = Uv::global().unwrap();
        let version_to_test = Version::from_str("3.8.20").unwrap();

        // Ensure it's not installed before the test
        let python_instances = uv.get_python_instances().unwrap();
        if let Some(instance) = python_instances
            .into_iter()
            .find(|i| i.version == version_to_test && i.path.is_some())
        {
            uv.uninstall_python_instance(instance).unwrap();
        }

        // Install
        uv.install_python_version(version_to_test.clone()).unwrap();
        let instances_after_install = uv.get_python_instances().unwrap();
        let installed_instance = instances_after_install
            .into_iter()
            .find(|i| i.version == version_to_test && i.path.is_some());
        assert!(installed_instance.is_some());

        // Uninstall
        uv.uninstall_python_version(version_to_test.clone())
            .unwrap();
        let instances_after_uninstall = uv.get_python_instances().unwrap();
        let uninstalled_instance = instances_after_uninstall
            .into_iter()
            .find(|i| i.version == version_to_test && i.path.is_some());
        assert!(uninstalled_instance.is_none());
    }

    #[test]
    #[serial]
    fn test_create_venv() {
        let uv = Uv::global().unwrap();

        // Install Python version 3.8.20 if not already installed
        let version_to_test = Version::from_str("3.8.20").unwrap();
        let python_instances = uv.get_python_instances().unwrap();
        let needs_install = !python_instances
            .iter()
            .any(|i| i.version == version_to_test && i.path.is_some());

        if needs_install {
            uv.install_python_version(version_to_test.clone()).unwrap();
        }

        // Create a virtual environment
        let venv_path = std::env::temp_dir().join("test_venv");
        let venv = uv.create_venv(&venv_path, version_to_test).unwrap();

        // Verify the virtual environment was created
        assert_eq!(venv.path, venv_path);
        assert_eq!(venv.python_instance.version, version_to_test);

        // Clean up
        std::fs::remove_dir_all(&venv_path).unwrap_or_else(|_| ());
    }

    #[test]
    #[serial]
    fn test_create_shell() {
        let uv = Uv::global().unwrap();

        // Install Python version 3.8.20 if not already installed
        let version_to_test = Version::from_str("3.8.20").unwrap();
        let python_instances = uv.get_python_instances().unwrap();
        let needs_install = !python_instances
            .iter()
            .any(|i| i.version == version_to_test && i.path.is_some());

        if needs_install {
            uv.install_python_version(version_to_test.clone()).unwrap();
        }

        // Create a virtual environment
        let venv_path = std::env::temp_dir().join("test_venv_shell");
        let venv = uv.create_venv(&venv_path, version_to_test).unwrap();

        // Test creating shell - this should succeed without errors
        let shell = venv.create_shell();
        assert!(shell.is_ok());

        // Verify the shell has access to Python binaries in the virtual environment
        let shell = shell.unwrap();

        // Test that we can execute a command in the shell context
        // Note: This is a basic test, in a real scenario we'd want more comprehensive testing
        // TODO: after implementing run_python_script, create an example python script in temp dir and try running it with venv.

        // Clean up
        std::fs::remove_dir_all(&venv_path).unwrap_or_else(|_| ());
    }
}
