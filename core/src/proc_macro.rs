//! proc-macro entrypoint.

use crate::generator::{BindingGenerator, ExternGenerator, ProjectGenerator};
use ligen_core::{
    ir::{Attributes, Implementation},
    proc_macro::Context,
    utils::Logger,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::{convert::TryFrom, fs::create_dir_all};

/// Generator entry point
pub fn ligen_c(context: Context, args: TokenStream, input: TokenStream) -> TokenStream {
    //println!("context: {:#?}", context);

    create_dir_all("./target/ligen/").expect("Failed to create target directory for the header");
    ProjectGenerator::generate();

    let attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    if let Ok(implementation) = Implementation::try_from(input) {
        output.append_all(ExternGenerator::generate(implementation.clone()));
        BindingGenerator::new(&attributes).generate(implementation.clone(), context);
    } else {
        Logger::log("Not supported.");
    }

    quote! {#output}
}
