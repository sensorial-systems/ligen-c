use ligen_core::ir::{Identifier, Parameter, Type, Function};
use ligen_core::ir::Implementation;
use ligen_core::ir::ImplementationItem::Constant;
use ligen_core::ir::ImplementationItem::Method;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use crate::Context;

#[derive(Debug, Copy, Clone)]
/// Extern generator.
pub struct ExternGenerator {}

impl ExternGenerator {
    /// Generate the function parameters.
    pub fn generate_parameters(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let type_ = Self::to_marshal_type(&parameter.type_);
                let identifier = &parameter.identifier;
                tokens.append_all(quote!{#identifier: #type_,});
                tokens
            })
    }

    /// Generate the function call arguments and its conversions.
    pub fn generate_arguments(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let identifier = &parameter.identifier;
                tokens.append_all(quote!{#identifier,});
                tokens
            })
    }

    pub fn to_marshal_type(type_: &Type) -> TokenStream {
        match type_ {
            Type::Compound(identifier) => quote! { *mut #type_ },
            _ => quote! { #type_ }
        }
    }

    pub fn from_marshal_type(type_: &Type) -> TokenStream {
        quote! { #type_ }
    }

    /// Generate the function output.
    pub fn generate_output(_context: &Context, output: &Option<Type>) -> TokenStream {
        match output {
            Some(type_) => Self::to_marshal_type(type_),
            _ => quote!{()}
        }
    }

    /// Generate the function
    pub fn generate_function_signature(context: &Context, implementation: &Implementation, method: &Function) -> TokenStream {
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
    pub fn generate_function_block(context: &Context, implementation: &Implementation, method: &Function) -> TokenStream {
        let arguments = Self::generate_arguments(context, &method.inputs);
        let self_identifier = &implementation.self_;
        let method_identifier = &method.identifier;
        // FIXME: This should be generalized somewhere.
        let result = if let Some(Type::Compound(identifier)) = method.output.as_ref() {
            quote! {
                Box::into_raw(Box::new(result))
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
    pub fn generate_function(context: &Context, implementation: &Implementation, method: &Function) -> TokenStream {
        let function_signature = Self::generate_function_signature(context, implementation, method);
        let method_block = Self::generate_function_block(context, implementation, method);
        quote! { #function_signature #method_block }
    }

    /// Generate externs for Constants and Methods.
    pub fn generate(context: &Context, implementation: &Implementation) -> TokenStream {
        implementation.items.iter().fold(TokenStream::new(), |mut tokens, item| {
            match item {
                Constant(_) => unimplemented!("Constants aren't implemented yet."),
                Method(method) => {
                    let method = Self::generate_function(context, implementation, method);
                    println!("{}", method);
                    tokens.append_all(method);
                }
            }
            tokens
        })
    }
}
