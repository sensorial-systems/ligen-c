//! Generator Module

mod files;

use ligen::ir::Attributes;
use ligen::generator::Context;
use ligen::generator::GenericFFIGenerator;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Generator;

impl ligen::generator::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Default::default()
    }
}

impl GenericFFIGenerator for Generator {}
