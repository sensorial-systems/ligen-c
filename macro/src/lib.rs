//! C Binding Generator.

#![cfg_attr(cargo_ligen, feature(proc_macro_span))]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use proc_macro::TokenStream;

/// Entry point for ligen_c
#[cfg(cargo_ligen)]
#[proc_macro_attribute]
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let source_file = proc_macro::Span::call_site().source_file();
    let source_file = ligen_c_core::SourceFile {
        is_real: source_file.is_real(),
        path: source_file.path(),
    };
    let arguments =
        ligen_core::proc_macro::Arguments::from_env().expect("Failed to get the arguments");
    let context = ligen_c_core::Context {
        source_file,
        arguments,
    };
    ligen_c_core::ligen_c(context, args.into(), input.into()).into()
}

use ligen_core::ir::Attributes;

#[cfg(cargo_ligen)]
#[proc_macro]
/// Ligen_c Project generator macro
pub fn ligen_c_package(args: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = args.into();
    let args = Attributes::try_from(tokens).expect("Failed to parse Attributes.");

    let source_file = proc_macro::Span::call_site().source_file();
    let source_file = ligen_c_core::SourceFile {
        is_real: source_file.is_real(),
        path: source_file.path(),
    };
    let arguments =
        ligen_core::proc_macro::Arguments::from_env().expect("Failed to get the arguments");
    let context = ligen_c_core::Context {
        source_file,
        arguments,
    };
    ligen_c_core::generator::ProjectGenerator::generate(&context, args);
    TokenStream::new()
}

/// Entry point for ligen_c
#[cfg(not(cargo_ligen))]
#[proc_macro_attribute]
pub fn ligen_c(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[cfg(not(cargo_ligen))]
#[proc_macro]
/// Ligen_c Project generator macro
pub fn ligen_c_package(_args: TokenStream) -> TokenStream {
    TokenStream::new()
}
