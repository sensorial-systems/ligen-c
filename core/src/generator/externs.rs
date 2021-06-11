use std::os::raw::c_char;

use crate::Context;
use ligen_core::ir::ImplementationItem::Constant;
use ligen_core::ir::ImplementationItem::Method;
use ligen_core::ir::{Function, Identifier, Parameter, Type};
use ligen_core::ir::{Implementation, Visibility};
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;

#[derive(Debug, Copy, Clone)]
/// Extern generator.
pub struct ExternGenerator {}

impl ExternGenerator {
    /// Generate the function parameters.
    pub fn generate_parameters(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let type_ = Self::to_marshal_parameter(&parameter.type_);
                let identifier = &parameter.identifier;
                tokens.append_all(quote! {#identifier: #type_,});
                tokens
            })
    }

    /// Generate the function call arguments and its conversions.
    pub fn generate_arguments(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let identifier = &parameter.identifier;
                tokens.append_all(quote! {#identifier.into(),});
                tokens
            })
    }

    /// Marshal type.
    pub fn to_marshal_output(type_: &Type) -> TokenStream {
        match type_ {
            Type::Compound(_identifier) => match _identifier.name.as_str() {
                "String" => quote! { *mut crate::ffi::RString },

                _ => quote! { *mut #type_ },
            },
            _ => quote! { #type_ },
        }
    }

    /// Marshal type.
    pub fn to_marshal_parameter(type_: &Type) -> TokenStream {
        match type_ {
            Type::Compound(_identifier) => match _identifier.name.as_str() {
                "String" => quote! { crate::ffi::CChar },
                _ => quote! { *mut #type_ },
            },
            _ => quote! { #type_ },
        }
    }

    /// Marshal type.
    pub fn from_marshal_type(type_: &Type) -> TokenStream {
        quote! { #type_ }
    }

    /// Generate the function output.
    pub fn generate_output(_context: &Context, output: &Option<Type>) -> TokenStream {
        match output {
            Some(type_) => Self::to_marshal_output(type_),
            _ => quote! {()},
        }
    }

    /// Generate the function
    pub fn generate_function_signature(
        context: &Context,
        implementation: &Implementation,
        method: &Function,
    ) -> TokenStream {
        let parameters = Self::generate_parameters(context, &method.inputs);
        let output = Self::generate_output(context, &method.output);
        let function_name = format!("{}_{}", implementation.self_.name, method.identifier.name);
        let function_identifier = Identifier::new(&function_name);
        quote! {
            #[no_mangle]
            pub extern fn #function_identifier(#parameters) -> #output
        }
    }

    /// Generate the function
    pub fn generate_function_block(
        context: &Context,
        implementation: &Implementation,
        method: &Function,
    ) -> TokenStream {
        let arguments = Self::generate_arguments(context, &method.inputs);
        let self_identifier = &implementation.self_;
        let method_identifier = &method.identifier;
        // FIXME: This should be generalized somewhere.
        let result = if let Some(Type::Compound(_identifier)) = method.output.as_ref() {
            quote! {
                Box::into_raw(Box::new(result.into()))
            }
        } else {
            quote! {result}
        };
        quote! {
            {
                let result = #self_identifier::#method_identifier(#arguments);
                #result
            }
        }
    }

    /// Generate an extern function for an implementation method.
    pub fn generate_function(
        context: &Context,
        implementation: &Implementation,
        method: &Function,
    ) -> TokenStream {
        let function_signature = Self::generate_function_signature(context, implementation, method);
        let method_block = Self::generate_function_block(context, implementation, method);
        quote! { #function_signature #method_block }
    }

    /// Generate destroy extern.
    pub fn generate_destroy(object_name: &Identifier) -> TokenStream {
        let destroy_name = Identifier::new(format!("{}_destroy", object_name.name).as_str());
        quote! {
            #[no_mangle]
            pub unsafe extern fn #destroy_name(object: *mut #object_name) {
                Box::from_raw(object);
            }
        }
    }

    /// Generate externs for Constants and Methods.
    pub fn generate(context: &Context, implementation: &Implementation) -> TokenStream {
        let mut tokens =
            implementation
                .items
                .iter()
                .fold(TokenStream::new(), |mut tokens, item| match item {
                    Constant(_) => unimplemented!("Constants aren't implemented yet."),
                    Method(method) => {
                        if let Visibility::Public = method.vis {
                            let method = Self::generate_function(context, implementation, method);
                            tokens.append_all(method);
                        }
                        tokens
                    }
                });
        tokens.append_all(ExternGenerator::generate_destroy(&implementation.self_));
        //println!("tokens: {}", tokens.to_string());
        tokens
    }
}
