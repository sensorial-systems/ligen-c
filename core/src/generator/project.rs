use std::fs::File;
use std::io::Write;
use ligen_core::ir::{Attribute, Attributes, Literal};
use ligen_core::generator::Context;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct ProjectGenerator {}

impl ProjectGenerator {
    /// generate function for the ProjectGenerator
    pub fn generate(context: &Context, attributes: &Attributes) {
        let target_dir = &context.arguments.target_dir;

        let generator_version = env!("CARGO_PKG_VERSION");

        let project_name = &context.arguments.crate_name;

        let crate_path = target_dir.join("ligen").join(&project_name);

        if attributes
            .attributes
            .iter()
            .any(|attr| *attr == Attribute::Literal(Literal::String(String::from("cmake"))))
        {
            let cmake_path = crate_path.join("CMakeLists.txt");
            let mut file = File::create(&cmake_path)
                .expect(&format!("Failed to create {}", cmake_path.display()));

            let cmake_content = format!(
                include_str!("CMakeLists.txt"),
                generator_version = generator_version,
                project_name = project_name
            );

            file.write_all(cmake_content.as_bytes())
                .expect("Failed to write file");
        }
    }
}
