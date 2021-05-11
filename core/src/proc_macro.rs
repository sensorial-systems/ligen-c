//! proc-macro entrypoint.

use crate::generator::ExternGenerator;
use ligen_core::{
    ir::{Attributes, Implementation},
    utils::Logger,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::convert::TryFrom;

/// Generator entry point
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let _attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    if let Ok(implementation) = Implementation::try_from(input) {
        output.append_all(ExternGenerator::generate(implementation))
    } else {
        Logger::log("Not supported.");
    }

    quote! {#output}
}
