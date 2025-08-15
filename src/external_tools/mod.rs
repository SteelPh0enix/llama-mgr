use std::path::PathBuf;

pub mod cmake;
pub mod ninja;
pub mod uv;
pub mod version;


pub trait ExternalTool {
    /// Create new instance of an external tool.
    /// Accepts path to tool's executable as argument.
    fn new(path: PathBuf) -> Self;

    /// Create new instance of an external tool.
    /// Path is looked up with `which` equivalent.
    fn global() -> Result<Self, which::Error>
    where
        Self: Sized;

    /// Returns `true` if tool is available, `false` otherwise.
    fn is_available(&self) -> bool;
}
