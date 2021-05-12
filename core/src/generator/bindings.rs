use ligen_core::{
    ir::{
        Implementation,
        ImplementationItem::{Constant, Method},
    },
    utils::Logger,
};

use crate::ast::Type;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct BindingGenerator {}

impl BindingGenerator {
    /// log function for the Logger struct
    pub fn generate(implementation: Implementation) -> Vec<String> {
        let mut statements = vec![
            String::from("#pragma once"),
            String::from("#include <stdint.h>"),
            String::from("#ifdef __cplusplus"),
            String::from("extern \"C\" {"),
            String::from("#endif"),
            String::from(format!("struct {} {}", implementation.self_.name, "{")),
            String::from("void* self;"),
            String::from("}"),
        ];

        for item in implementation.items {
            match item {
                Constant(_) => Logger::log("Const extern not supported."),
                Method(method) => {
                    let name =
                        format!("{}_{}", &implementation.self_.name, &method.identifier.name);

                    let mut inner_types: Vec<String> = vec![];
                    method.input.into_iter().for_each(|parameter| {
                        let ident = parameter.identifier.name;
                        let typ = parameter.type_;
                        inner_types.push(format!("{} {}", format!("{}", Type::from(typ)), ident));
                    });

                    statements.push(String::from(format!(
                        "{} {}({});",
                        if let Some(typ) = method.output {
                            format!("{}", Type::from(typ))
                        } else {
                            String::from("void")
                        },
                        &name,
                        inner_types.join(", ")
                    )));
                }
            };
        }

        statements.extend_from_slice(&[
            String::from("#ifdef __cplusplus"),
            String::from("}"),
            String::from("#endif"),
        ]);
        statements
    }
}
