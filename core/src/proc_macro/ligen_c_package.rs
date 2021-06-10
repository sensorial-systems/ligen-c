//! Project generator entry point module.

use crate::generator::ProjectGenerator;
use crate::generator::FFI;
use ligen_core::ir::Attributes;
use ligen_core::proc_macro::Context;
use proc_macro2::TokenStream;
use std::convert::TryFrom;

/// Project generator entry point
pub fn ligen_c_package(context: Context, args: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Couldn't get attributes.");
    ProjectGenerator::generate(&context, args);
    FFI::generate_rstring()
}
