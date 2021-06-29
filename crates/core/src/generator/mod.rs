//! Generator Module

mod ffi;
mod project;
mod files;

pub use ffi::*;
pub use project::*;

use ligen::ir::Attributes;
use ligen::generator::Context;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Generator;

impl ligen::generator::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Default::default()
    }
}