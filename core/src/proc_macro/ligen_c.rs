//! Generator entry point module.

use crate::generator::{BindingGenerator, ExternGenerator};
use ir::processing::ReplaceIdentifier;
use ir::Attributes;
use ir::Implementation;
use ligen_core::ir;
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
    if let Ok(mut implementation) = Implementation::try_from(input) {
        let id = implementation.self_.clone();
        let mut lower_case_id = id.clone();
        lower_case_id.name = lower_case_id.name.to_lowercase();
        implementation.replace_identifier(&ir::Identifier::from("Self"), &id);
        implementation.replace_identifier(&ir::Identifier::from("self"), &lower_case_id);
        output.append_all(ExternGenerator::generate(&context, &implementation));
        BindingGenerator::new(&attributes).generate(&context, &implementation);
    } else {
        Logger::log("Not supported.");
    }
    quote! {#output}
}
