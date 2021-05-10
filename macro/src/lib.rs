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
#[proc_macro_attribute]
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    ligen_c_core::ligen_c(args.into(), input.into()).into()
}
