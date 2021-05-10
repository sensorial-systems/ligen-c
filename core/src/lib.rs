// use ligen_core::

pub mod ast;

use proc_macro2::TokenStream;

pub fn ligen_c(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}