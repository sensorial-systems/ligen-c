#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

// use ligen_core::

pub mod ast;

use proc_macro2::TokenStream;

pub fn ligen_c(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}