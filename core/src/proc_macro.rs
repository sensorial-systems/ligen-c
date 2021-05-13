//! proc-macro entrypoint.

use crate::generator::{BindingGenerator, ExternGenerator};
use ligen_core::{
    ir::{Attributes, Implementation},
    utils::Logger,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::{
    convert::TryFrom,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

/// Generator entry point
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    create_dir_all("./target/ligen/").expect("Failed to create target directory for the header");
    let path = Path::new("./target/ligen/test.h");
    let attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let mut output = input.clone();
    if let Ok(implementation) = Implementation::try_from(input) {
        output.append_all(ExternGenerator::generate(implementation.clone()));

        let bindings = BindingGenerator::new(&attributes).generate(implementation.clone());
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(bindings.join("\n").as_bytes())
            .expect("Failed to write file");
    } else {
        Logger::log("Not supported.");
    }

    quote! {#output}
}
