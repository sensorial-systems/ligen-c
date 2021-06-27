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
use ligen::ir;
use ligen::generator::{Context, File};
use ligen::generator::FileSet;
use ligen::generator::FileGenerator;
use ligen::prelude::*;
use std::path::PathBuf;

/// Generator structure.
#[derive(Clone, Copy, Debug)]
pub struct Generator {
    binding_generator: BindingGenerator
}

impl ligen::generator::Generator for Generator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let binding_generator = BindingGenerator::new(attributes);
        Self { binding_generator }
    }

    fn generate_ffi(&self, context: &Context, implementation: Option<&Implementation>) -> TokenStream {
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

impl ligen::generator::FileGenerator for Generator {
    fn generate_file_set(&self, _context: &Context, _file_set: &mut FileSet) {}
}
impl ligen::generator::FFIGenerator  for Generator {}