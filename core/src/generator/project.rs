use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use crate::Context;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct ProjectGenerator {}

impl ProjectGenerator {
    /// generate function for the ProjectGenerator
    pub fn generate(context: &Context) {
        let target_dir = &context.arguments.target_dir;

        let generator_version = env!("CARGO_PKG_VERSION");

        let name = &context.arguments.crate_name;

        let crate_path = target_dir.join("ligen").join(&name);

        let cmake_path = crate_path.join("CMakeLists.txt");
        let mut file =
            File::create(&cmake_path).expect(&format!("Failed to create {}", cmake_path.display()));

        let cmake_content = format!(
            "# Auto-generated by ligen-c {}

CMAKE_MINIMUM_REQUIRED(VERSION 3.0)
PROJECT(\"{}\")

IF(TARGET ${{PROJECT_NAME}})
RETURN()
ENDIF()

# INTERFACE is used to create a header-only library.
ADD_LIBRARY(${{PROJECT_NAME}} INTERFACE)
TARGET_INCLUDE_DIRECTORIES(${{PROJECT_NAME}} INTERFACE include)
TARGET_LINK_LIBRARIES(${{PROJECT_NAME}} INTERFACE \"${{CMAKE_CURRENT_SOURCE_DIR}}/lib/${{PROJECT_NAME}}.lib\")",
            generator_version, &name
        );

        file.write_all(cmake_content.as_bytes())
            .expect("Failed to write file");
    }
}
