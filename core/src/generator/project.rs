use ligen_core::ir::Attributes;
use ligen_core::generator::Context;
use ligen_core::generator::file::{FileSet, File};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct ProjectGenerator {}

impl ProjectGenerator {
    /// generate function for the ProjectGenerator
    pub fn generate(context: &Context, _attributes: &Attributes) -> FileSet {
        let mut file_set = FileSet::new();

        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &context.arguments.crate_name;

        let content = format!(
            include_str!("CMakeLists.txt"),
            generator_version = generator_version,
            project_name = project_name
        );
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.add(file);
        file_set
    }
}
