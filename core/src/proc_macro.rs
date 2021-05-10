//! proc-macro entrypoint.

use ligen_core::ir::{Attributes, Implementation};

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use std::fmt::Display;

//TODO: Move to ligen_core
pub struct Logger {}

impl Logger {
    pub fn log<D: Display>(d: D) {
        println!("{}", d);
    }
}

/// Generator entry point
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    let _attributes = Attributes::try_from(args).expect("Couldn't get attributes.");
    let output = input.clone();
    if let Ok(_implementation) = Implementation::try_from(input) {
    } else {
        Logger::log("Not supported.");
    }

    output
}