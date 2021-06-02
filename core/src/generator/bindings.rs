use crate::ast::Type;

use ligen_core::ir::Attribute;
use ligen_core::ir::Attributes;
use ligen_core::ir::Implementation;
use ligen_core::ir::ImplementationItem::Constant;
use ligen_core::ir::ImplementationItem::Method;
use ligen_core::proc_macro::Context;
use ligen_core::utils::Logger;

use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct BindingGenerator {
    sized_integer: bool,
}

impl Default for BindingGenerator {
    fn default() -> Self {
        Self {
            sized_integer: false,
        }
    }
}

impl BindingGenerator {
    /// function to create a new BindingGenerator
    pub fn new(attr: &Attributes) -> Self {
        let mut sized_integer = false;
        if attr.attributes.iter().any(|attribute| {
            if let Attribute::Named(ident, lit) = attribute {
                if (ident.name.as_str(), lit.to_string().as_str()) == ("integer", "sized") {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            sized_integer = true;
        }

        Self { sized_integer }
    }

    /// generate function for the BindingGenerator
    pub fn generate(&self, context: &Context, implementation: &Implementation) {
        let mut statements = vec![
            String::from("#pragma once"),
            String::from("#include <stdint.h>"),
            String::from("#ifdef __cplusplus"),
            String::from("extern \"C\" {"),
            String::from("#endif"),
            String::from(format!("typedef struct Struct_{} {{", implementation.self_.name)),
            String::from("void* self;"),
            String::from(format!("}} {};", implementation.self_.name)),
        ];

        for item in &implementation.items {
            match item {
                Constant(_) => Logger::log("Const extern not supported."),
                Method(method) => {
                    let name =
                        format!("{}_{}", &implementation.self_.name, &method.identifier.name);

                    let mut inner_types: Vec<String> = vec![];
                    method.input.iter().for_each(|parameter| {
                        let ident = &parameter.identifier.name;
                        let typ = &parameter.type_;
                        inner_types.push(format!(
                            "{} {}",
                            format!("{}", Type::from(typ.clone())),
                            ident
                        ));
                    });

                    //TODO: Distinguish sized types

                    statements.push(String::from(format!(
                        "{} {}({});",
                        if let Some(typ) = &method.output {
                            format!("{}", Type::from(typ.clone()))
                        } else {
                            String::from("void")
                        },
                        &name,
                        inner_types.join(", ")
                    )));
                }
            }
        }

        statements.extend_from_slice(&[
            String::from("#ifdef __cplusplus"),
            String::from("}"),
            String::from("#endif"),
        ]);

        let header_path = context
            .arguments
            .target_dir
            .join("ligen")
            .join(&context.arguments.crate_name)
            .join("include")
            .join(format!("{}.h", implementation.self_.name));

        let mut file = File::create(&header_path)
            .expect(&format!("Failed to create file {}.", header_path.display()));
        file.write_all(statements.join("\n").as_bytes())
            .expect("Failed to write file");
    }
}
