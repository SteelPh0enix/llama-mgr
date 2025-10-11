use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration structure for llama-mgr
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub config: ConfigSection,
    pub paths: PathsSection,
    pub profiles: HashMap<String, Profile>,
}

/// Configuration section
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSection {
    pub default_profile: String,
}

/// Paths section
#[derive(Debug, Serialize, Deserialize)]
pub struct PathsSection {
    pub llama_dir: PathBuf,
    pub models_dir: PathBuf,
}

/// Profile configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub cmake_args: Vec<String>,
    pub cmake_generator: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config: ConfigSection {
                default_profile: "vulkan".to_string(),
            },
            paths: PathsSection {
                llama_dir: PathBuf::from("./llama"),
                models_dir: PathBuf::from("./models"),
            },
            profiles: {
                let mut profiles = HashMap::new();
                
                // Default CPU profile
                profiles.insert("cpu".to_string(), Profile {
                    cmake_args: vec![
                        "-DGGML_CPU=ON".to_string(),
                        "-DGGML_LTO=ON".to_string(),
                    ],
                    cmake_generator: None,
                });
                
                // Default Vulkan profile
                profiles.insert("vulkan".to_string(), Profile {
                    cmake_args: vec![
                        "-DGGML_VULKAN=ON".to_string(),
                        "-DGGML_LTO=ON".to_string(),
                    ],
                    cmake_generator: None,
                });
                
                profiles
            },
        }
    }
}

impl Config {
    /// Get a profile by name, or return the default profile
    pub fn get_profile(&self, name: Option<&str>) -> Option<&Profile> {
        match name {
            Some(n) => self.profiles.get(n),
            None => self.profiles.get(&self.config.default_profile),
        }
    }
}
