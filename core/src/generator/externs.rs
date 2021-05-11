use ligen_core::{
    ir::{
        Identifier, Implementation,
        ImplementationItem::{Constant, Method},
    },
    utils::Logger,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct ExternGenerator {}

impl ExternGenerator {
    /// log function for the Logger struct
    pub fn generate(implementation: Implementation) -> TokenStream {
        let mut externs = TokenStream::new();
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
        externs
    }
}
