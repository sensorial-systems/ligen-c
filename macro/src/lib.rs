//! C Binding Generator.

#![feature(proc_macro_span)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use ligen_c_core::{Context, SourceFile};
use proc_macro::TokenStream;
use ligen_core::proc_macro::Arguments;

/// Entry point for ligen_c
#[proc_macro_attribute]
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let source_file = proc_macro::Span::call_site().source_file();
    let source_file = SourceFile {
        is_real: source_file.is_real(),
        path: source_file.path()
    };
    let arguments = Arguments::from_env();
    let context = Context { source_file, arguments };
    ligen_c_core::ligen_c(context, args.into(), input.into()).into()
}
