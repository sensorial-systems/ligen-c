use crate::{Context, Marshalling};
use ligen_core::ir::Identifier;
use ligen_core::ir::Implementation;
use ligen_core::ir::ImplementationItem::Constant;
use ligen_core::ir::ImplementationItem::Method;
use ligen_core::utils::Logger;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct ExternGenerator {}

impl ExternGenerator {
    /// log function for the Logger struct
    pub fn generate(_context: &Context, implementation: &Implementation) -> TokenStream {
        let mut externs = TokenStream::new();
        for item in &implementation.items {
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
                    method.input.iter().for_each(|parameter| {
                        let inner_arg: TokenStream = Marshalling::from(&parameter.type_)
                            .from_convert(parameter.identifier.clone());
                        let ident = parameter.identifier.to_token_stream();
                        inner_types.append_all(quote! {#inner_arg, });
                        let parameter_type = Marshalling::from(&parameter.type_).to_token_stream();
                        parameters.append_all(quote! {#ident: #parameter_type, })
                    });

                    let return_type = match &method.output.clone() {
                        Some(ty) => {
                            let typ = Marshalling::from(ty).to_token_stream();
                            quote! {#typ}
                        }
                        None => quote! {()},
                    };
                    let conversion =
                        Marshalling::from(&method.output.clone().unwrap()).to_convert();
                    let inner = quote! {
                        unsafe {
                            #self_name::#method_name(#inner_types)#conversion
                        }
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
