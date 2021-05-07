#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use ligen::ligen;
use proc_macro::TokenStream;

pub(crate) mod ast;

#[proc_macro_attribute]
/// Entry point for ligen_c
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}
