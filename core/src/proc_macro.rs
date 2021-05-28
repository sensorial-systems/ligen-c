//! proc-macro entrypoint.

use crate::generator::BindingGenerator;
use crate::generator::ExternGenerator;
use crate::generator::ProjectGenerator;
use ligen_core::ir::Attributes;
use ligen_core::ir::Implementation;
use ligen_core::proc_macro::Context;
use ligen_core::utils::Logger;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use std::convert::TryFrom;
use std::fs::create_dir_all;

/// Generator entry point
pub fn ligen_c(context: Context, args: TokenStream, input: TokenStream) -> TokenStream {
    create_dir_all("./target/ligen/").expect("Failed to create target directory for the header");
    ProjectGenerator::generate(&context);

    let attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    if let Ok(implementation) = Implementation::try_from(input) {
        output.append_all(ExternGenerator::generate(&context, &implementation));
        BindingGenerator::new(&attributes).generate(&context, &implementation);
    } else {
        Logger::log("Not supported.");
    }

    quote! {#output}
}
