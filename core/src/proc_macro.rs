//! proc-macro entrypoint.

use crate::generator::{BindingGenerator, ExternGenerator, ProjectGenerator};
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
    let target_dir_ligen = &context.arguments.target_dir.join("ligen");
    create_dir_all(target_dir_ligen).expect("Failed to create target directory for the header");

    let project_dir = target_dir_ligen.join(&context.arguments.crate_name);

    create_dir_all(project_dir.join("include")).expect("Failed to create include directory");
    create_dir_all(project_dir.join("lib")).expect("Failed to create lib directory");

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

/// Project generator entry point
pub fn ligen_c_package(context: Context, args: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Couldn't get attributes.");
    ProjectGenerator::generate(&context, args);

    TokenStream::new()
}
