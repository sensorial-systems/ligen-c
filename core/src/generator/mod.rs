//! Generator Module

mod bindings;
mod externs;
mod project;
mod ffi;

pub use bindings::*;
pub use externs::*;
pub use project::*;
pub use ffi::*;



use crate::generator::{BindingGenerator, ExternGenerator};
use ir::Attributes;
use ir::Implementation;
use ligen_core::ir;
use ligen_core::generator::Context;
use proc_macro2::TokenStream;
use ligen_core::generator::file::FileSet;

/// Generator structure.
#[derive(Clone, Copy, Debug)]
pub struct Generator {
    binding_generator: BindingGenerator
}

impl ligen_core::generator::Generator for Generator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let binding_generator = BindingGenerator::new(attributes);
        Self { binding_generator }
    }

    fn generate_externs(&self, context: &Context, implementation: Option<&Implementation>) -> TokenStream {
        implementation
            .map(|implementation| ExternGenerator::generate(context, implementation))
            .unwrap_or_else(|| TokenStream::new())
    }

    fn generate_files(&self, context: &Context, implementation: Option<&Implementation>) -> FileSet {
        implementation
            .map(|implementation| self.binding_generator.generate(context, implementation))
            .unwrap_or_default()
    }
}

/// CMake project generator.
#[derive(Debug, Clone)]
// FIXME: Rewrite it.
pub struct ProjectGenerator {
    attributes: Attributes
}

impl ligen_core::generator::Generator for ProjectGenerator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let attributes = attributes.clone();
        Self { attributes }
    }

    fn generate_externs(&self, _context: &Context, _implementation: Option<&Implementation>) -> TokenStream {
        ffi::FFI::generate_rstring()
    }

    fn generate_files(&self, context: &Context, _implementation: Option<&Implementation>) -> FileSet {
        project::ProjectGenerator::generate(context, &self.attributes)
    }
}
