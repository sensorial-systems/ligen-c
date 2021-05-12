//! proc-macro entrypoint.

use crate::generator::{BindingGenerator, ExternGenerator};
use ligen_core::{
    ir::{Attributes, Implementation},
    utils::Logger,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::{convert::TryFrom, fs::File, io::Write};

/// Generator entry point
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let _attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    if let Ok(implementation) = Implementation::try_from(input) {
        output.append_all(ExternGenerator::generate(implementation.clone()));
        let bindings = BindingGenerator::generate(implementation.clone());
        let mut file = File::create("./test.c").expect("Failed to create file");
        file.write_all(bindings.join("\n").as_bytes())
            .expect("Failed to write file");
    } else {
        Logger::log("Not supported.");
    }

    quote! {#output}
}

// pub fn ligen_c(args: TokenStream, item: TokenStream) -> TokenStream {
//     let attributes = Attributes::from(args);
//     let output = input.clone();
//     let ir = ImplBlock::from(item);
//     let c_generator = CGenerator::from(attributes);
//     let files = generator.generate_bindings(&ir);
//     files.save();
//     let externs: TokenStream = generator.generate_externs(&ir);
//     output.append(externs);
//     output
// }
