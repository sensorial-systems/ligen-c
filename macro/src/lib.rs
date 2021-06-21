//! C Binding Generator.

#![cfg_attr(cargo_ligen, feature(proc_macro_span))]

use ligen_core::proc_macro::prelude::*;

ligen::proc_macro!(name = "ligen_c_package", generator = "ligen_c_core::generator::ProjectGenerator");
// ligen::proc_macro_attribute!(name = "ligen_c", generator = "ligen_c_core::generator::Generator");

/// Generator proc-macro entry-point.
// #[cfg(cargo_ligen)]
#[proc_macro_attribute]
pub fn ligen_c(attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source_file = proc_macro::Span::call_site().source_file().into();
    let arguments = ligen_core::generator::Arguments::from_env().expect("Failed to get the arguments");
    let context = Context { source_file, arguments };
    let attributes: proc_macro2::TokenStream = attributes.into();
    let attributes = ligen_core::ir::Attributes::try_from(attributes).expect("Couldn't get attributes.");
    let input: proc_macro2::TokenStream = input.into();
    let mut output = input.clone();
    let implementation: Option<ir::Implementation> = ir::Implementation::try_from(input).ok();
    let implementation = implementation.map(|mut implementation| {
        let id = implementation.self_.clone();
        let mut lower_case_id = id.clone();
        lower_case_id.name = lower_case_id.name.to_lowercase();
        implementation.replace_identifier(&ir::Identifier::from("Self"), &id);
        implementation.replace_identifier(&ir::Identifier::from("self"), &lower_case_id);
        implementation
    });
    let generator = ligen_c_core::generator::Generator::new(&context, &attributes);
    output.append_all(generator.generate(&context, implementation.as_ref()));
    output.into()
}

// /// Generator proc-macro entry-point.
// #[cfg(not(cargo_ligen))]
// #[proc_macro_attribute]
// pub fn ligen_c(_attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     input
// }
