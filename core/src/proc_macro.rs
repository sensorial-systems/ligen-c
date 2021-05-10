//! proc-macro entrypoint.

use ligen_core::ir::{
    Attributes, Identifier, Implementation,
    ImplementationItem::{Constant, Method},
};

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
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
    let mut externs = TokenStream::new();
    if let Ok(implementation) = Implementation::try_from(input) {
        for item in implementation.items {
            match item {
                Constant(_) => Logger::log("Const extern not supported."),
                Method(method) => {
                    let self_name = Identifier::new(&implementation.self_.name);
                    let method_name = Identifier::new(&method.identifier.name);
                    let name = Identifier::new(
                        format!("{}_{}", &self_name.name, &method_name.name).as_str(),
                    );

                    let mut parameters = TokenStream::new();
                    let mut inner_types = TokenStream::new();
                    method.input.into_iter().for_each(|parameter| {
                        let ident = parameter.identifier.to_token_stream();
                        inner_types.append_all(quote! {#ident, });
                        let par = parameter.to_token_stream();
                        parameters.append_all(quote! {#par, })
                    });

                    let return_type = match method.output {
                        Some(ty) => {
                            let typ = ty.to_token_stream();
                            quote! {#typ}
                        }
                        None => quote! {()},
                    };
                    let inner = quote! {
                        #self_name::#method_name(#inner_types)
                    };

                    externs.append_all(quote! {
                    #[no_mangle]
                    pub extern fn #name(#parameters) -> #return_type {
                        #inner
                    }
                    })
                }
            };
        }
    } else {
        Logger::log("Not supported.");
    }

    quote! {
        #output
        #externs
    }
}
