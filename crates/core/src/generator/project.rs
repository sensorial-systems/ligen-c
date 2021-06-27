use ligen::prelude::*;
use ligen::ir::{Attributes, Implementation};
use ligen::generator::{Context, FileSet, FileGenerator};
use ligen::generator::File;
use std::path::PathBuf;

/// CMake project generator.
#[derive(Debug, Clone)]
// FIXME: Rewrite it.
pub struct ProjectGenerator {
    attributes: Attributes
}

impl ligen::generator::Generator for ProjectGenerator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let attributes = attributes.clone();
        Self { attributes }
    }

    fn generate_ffi(&self, _context: &Context, _implementation: Option<&Implementation>) -> TokenStream {
        super::ffi::FFI::generate_rstring()
    }

    fn generate_files(&self, context: &Context, _implementation: Option<&Implementation>) -> FileSet {
        let mut file_set = FileSet::default();
        self.generate_file_set(&context, &mut file_set);
        file_set
    }
}

impl ligen::generator::FileGenerator for ProjectGenerator {
    fn generate_file_set(&self, context: &Context, file_set: &mut FileSet) {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &context.arguments.crate_name;

        let content = format!(
            include_str!("CMakeLists.txt"),
            generator_version = generator_version,
            project_name = project_name
        );
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.add(file);
    }
}

impl ligen::generator::FFIGenerator for ProjectGenerator {}