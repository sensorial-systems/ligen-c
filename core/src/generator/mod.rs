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

    fn generate_externs(&self, context: &Context, implementation: &Implementation) -> TokenStream {
        ExternGenerator::generate(context, implementation)
    }

    fn generate_files(&self, context: &Context, implementation: &Implementation) {
        self.binding_generator.generate(context, implementation)
    }
}
