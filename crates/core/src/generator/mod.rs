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
use ligen::generator::{Context, FFIGenerator, File};
use ligen::generator::FileSet;
use ligen::generator::FileGenerator;
use ligen::prelude::*;
use ligen::ir::ImplementationItem;
use std::path::PathBuf;

/// Generator structure.
#[derive(Clone, Copy, Debug)]
pub struct Generator {
    binding_generator: BindingGenerator
}

impl Generator {
    fn path(&self, implementation: &Implementation) -> PathBuf {
        PathBuf::from("include").join(format!("{}.h", implementation.self_.name))
    }
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
}

impl FileGenerator for Generator {
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, implementation: Option<&Implementation>) {
        if let Some(implementation) = implementation {
            let mut content = self.binding_generator.generate_prelude(context);
            content.push_line(self.binding_generator.generate_struct(context, implementation));

            for item in &implementation.items {
                match item {
                    ImplementationItem::Constant(_) => todo!("Const extern not supported."),
                    ImplementationItem::Method(method) => content.push_line(self.binding_generator.generate_function(context, implementation, method)),
                }
            }

            content.push_line(BindingGenerator::generate_drop(
                &implementation.self_.name,
            ));

            content.push_line(self.binding_generator.generate_epilogue(context));

            let file = File::new(self.path(implementation), content);
            file_set.add(file);
        }
    }
}

impl FFIGenerator for Generator {}