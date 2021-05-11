//! proc-macro entrypoint.

use crate::generator::ExternGenerator;
use ligen_core::ir::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::convert::TryFrom;

/// Generator entry point
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let _attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    let externs = ExternGenerator::generate(input);
    if let Some(generated) = externs {
        output.append_all(generated)
    }

    quote! {#output}
}
