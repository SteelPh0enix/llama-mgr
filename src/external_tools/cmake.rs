use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use crate::external_tools::ExternalTool;

pub struct CMake {
    path: PathBuf,
}

impl CMake {
    pub fn generate<T: AsRef<Path>>(
        &self,
        source_dir: T,
        build_dir: T,
        generator: T,
        install_dir: Option<T>,
        additional_args: Option<&[T]>,
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("-S")
            .arg(source_dir.as_ref())
            .arg("-B")
            .arg(build_dir.as_ref())
            .arg("-G")
            .arg(generator.as_ref());

        install_dir.inspect(|d| {
            command.arg(format!(
                "-DCMAKE_INSTALL_PREFIX={}",
                d.as_ref().to_string_lossy()
            ));
        });

        additional_args.inspect(|args| {
            args.iter().for_each(|arg| {
                command.arg(arg.as_ref());
            })
        });

        command.status()
    }

    pub fn build<T: AsRef<Path>>(
        &self,
        build_dir: T,
        config: T,
        additional_args: Option<&[T]>,
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("--build")
            .arg(build_dir.as_ref())
            .arg("--config")
            .arg(config.as_ref());

        additional_args.inspect(|args| {
            args.iter().for_each(|arg| {
                command.arg(arg.as_ref());
            })
        });

        command.status()
    }

    pub fn install<T: AsRef<Path>>(
        &self,
        build_dir: T,
        config: T,
        additional_args: Option<&[T]>,
    ) -> std::io::Result<ExitStatus> {
        let mut command = Command::new(&self.path);
        command
            .arg("--install")
            .arg(build_dir.as_ref())
            .arg("--config")
            .arg(config.as_ref());

        additional_args.inspect(|args| {
            args.iter().for_each(|arg| {
                command.arg(arg.as_ref());
            })
        });

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
    use std::fs;
    use tempfile::TempDir;

    const CMAKE_LISTS_TXT: &str = r#"
cmake_minimum_required(VERSION 3.10)
project(test_project)

# Set default value for CMAKE_TEST_VAR if not provided
if(NOT DEFINED CMAKE_TEST_VAR)
    set(CMAKE_TEST_VAR "DefaultTestValue")
endif()

add_executable(test_app main.cpp)

install(TARGETS test_app DESTINATION bin)

# Add header generation based on CMAKE_TEST_VAR
configure_file(
    test_config.h.in
    ${CMAKE_CURRENT_BINARY_DIR}/test_config.h
    @ONLY
)
target_include_directories(test_app PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
"#;

    const MAIN_CPP: &str = r#"
#include <iostream>
#include "test_config.h"

int main() {
    std::cout << "Hello from CMake test!" << std::endl;
    std::cout << "Test value from config: " << TEST_VALUE << std::endl;
    return 0;
}
"#;

    const TEST_CONFIG_H_IN: &str = r#"
#pragma once

#define TEST_VALUE "@CMAKE_TEST_VAR@"
"#;

    fn create_test_project(temp_dir: &TempDir) -> (PathBuf, PathBuf) {
        let source_dir = temp_dir.path().join("src");
        let build_dir = temp_dir.path().join("build");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&build_dir).unwrap();

        let cmake_lists_path = source_dir.join("CMakeLists.txt");
        let main_cpp_path = source_dir.join("main.cpp");
        let test_config_h_in_path = source_dir.join("test_config.h.in");

        fs::write(&cmake_lists_path, CMAKE_LISTS_TXT).unwrap();
        fs::write(&main_cpp_path, MAIN_CPP).unwrap();
        fs::write(&test_config_h_in_path, TEST_CONFIG_H_IN).unwrap();

        (source_dir, build_dir)
    }

    #[test]
    fn test_is_available() {
        let tool = CMake::global();
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert!(tool.is_available());
    }

    #[test]
    fn test_generate() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        let result = cmake.generate(&source_dir, &build_dir, &PathBuf::from("Ninja"), None, None);

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Check that CMake generated files exist
        assert!(fs::metadata(build_dir.join("CMakeCache.txt")).is_ok());
        assert!(fs::metadata(build_dir.join("CMakeFiles")).is_ok());
    }

    #[test]
    fn test_generate_with_additional_args() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        let result = cmake.generate(
            &source_dir,
            &build_dir,
            &PathBuf::from("Ninja"),
            None,
            Some(&[&PathBuf::from("-DCMAKE_TEST_VAR=HelloFromTest")]),
        );

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());
    }

    #[test]
    fn test_generate_with_additional_args_header_generation() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        // Generate with additional args that will create a header file
        let result = cmake.generate(
            &source_dir,
            &build_dir,
            &PathBuf::from("Ninja"),
            None,
            Some(&[&PathBuf::from("-DCMAKE_TEST_VAR=HelloFromTest")]),
        );

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Check that the configured header file was created
        let config_header_path = build_dir.join("test_config.h");
        assert!(fs::metadata(&config_header_path).is_ok());

        // Check that the header content is correct
        let header_content = fs::read_to_string(&config_header_path).unwrap();
        assert!(header_content.contains("#pragma once"));
        assert!(header_content.contains("#define TEST_VALUE \"HelloFromTest\""));
    }

    #[test]
    fn test_generate_with_default_header_value() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        // Generate without additional args - should use default value
        let result = cmake.generate(
            &source_dir,
            &build_dir,
            &PathBuf::from("Ninja"),
            None,
            None,
        );

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Check that the configured header file was created with default value
        let config_header_path = build_dir.join("test_config.h");
        assert!(fs::metadata(&config_header_path).is_ok());

        // Check that the header content has the default value
        let header_content = fs::read_to_string(&config_header_path).unwrap();
        assert!(header_content.contains("#pragma once"));
        assert!(header_content.contains("#define TEST_VALUE \"DefaultTestValue\""));
    }

    #[test]
    fn test_build() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        // First generate the build system
        cmake
            .generate(&source_dir, &build_dir, &PathBuf::from("Ninja"), None, None)
            .unwrap();

        // Then build the project
        let result = cmake.build(&build_dir, &PathBuf::from("Debug"), None);

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Check that the executable was created
        let exe_name = if cfg!(target_os = "windows") {
            "test_app.exe"
        } else {
            "test_app"
        };
        assert!(fs::metadata(build_dir.join(exe_name)).is_ok());
    }

    #[test]
    fn test_build_with_additional_args() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);

        // First generate the build system
        cmake
            .generate(&source_dir, &build_dir, &PathBuf::from("Ninja"), None, None)
            .unwrap();

        // Then build with additional args
        let result = cmake.build(
            &build_dir,
            &PathBuf::from("Debug"),
            Some(&[&PathBuf::from("--"), &PathBuf::from("-j2")]), // Limit to 2 parallel jobs
        );

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());
    }

    use std::io;
    use std::path::Path;
    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_install() {
        let temp_dir = TempDir::new().unwrap();
        let cmake = CMake::global().unwrap();
        let (source_dir, build_dir) = create_test_project(&temp_dir);
        let install_dir = temp_dir.path().join("install");

        // First generate and build
        cmake
            .generate(
                &source_dir,
                &build_dir,
                &PathBuf::from("Ninja"),
                Some(&install_dir),
                None,
            )
            .unwrap();

        cmake
            .build(&build_dir, &PathBuf::from("Debug"), None)
            .unwrap();

        // Then install
        let result = cmake.install(&build_dir, &PathBuf::from("Debug"), None);

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Check that installation directory was created
        assert!(fs::metadata(&install_dir).is_ok());

        // Check for typical installation artifacts
        let bin_dir = install_dir.join("bin");
        if fs::metadata(&bin_dir).is_ok() {
            let exe_name = if cfg!(target_os = "windows") {
                "test_app.exe"
            } else {
                "test_app"
            };
            assert!(fs::metadata(bin_dir.join(exe_name)).is_ok());
        }
    }

    #[test]
    fn test_cmake_new() {
        let path = PathBuf::from("/usr/bin/cmake"); // Doesn't need to exist for this test
        let cmake = CMake::new(path.clone());
        assert_eq!(cmake.path, path);
    }

    #[test]
    fn test_cmake_is_available() {
        let cmake = CMake::global().unwrap();
        assert!(cmake.is_available());
    }
}
