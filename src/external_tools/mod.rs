use std::path::PathBuf;

pub mod cmake;
pub mod ninja;
pub mod uv;

use which;

pub trait ExternalTool {
    /// Create new instance of an external tool.
    /// Accepts path to tool's executable as argument.
    fn new(path: PathBuf) -> Self;

    /// Create new instance of an external tool.
    /// Accepts name of tool's executable.
    /// Path is looked up with `which` equivalent.
    fn new_which(name: &str) -> Result<Self, which::Error>
    where
        Self: Sized,
    {
        which::which(name).map(|path: PathBuf| Self::new(path))
    }

    /// Returns `true` if tool is available, `false` otherwise.
    fn is_available(&self) -> bool;
}
