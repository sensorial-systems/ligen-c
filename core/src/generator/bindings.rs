use crate::ast::Type;

use ligen_core::ir;
use ligen_core::ir::{Attribute, Function};
use ligen_core::ir::Attributes;
use ligen_core::ir::Implementation;
use ligen_core::ir::ImplementationItem::Constant;
use ligen_core::ir::ImplementationItem::Method;
use ligen_core::proc_macro::Context;
use ligen_core::utils::Logger;

use std::fs::File;
use std::io::Write;
use std::ops::Add;

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

pub trait StringExt {
    fn push_line<String: AsRef<str>>(&mut self, line: String);
    fn push_token<String: AsRef<str>>(&mut self, token: String);
}

impl StringExt for String {
    fn push_line<String: AsRef<str>>(&mut self, line: String) {
        self.push_str(line.as_ref());
        self.push('\n');
    }
    fn push_token<String: AsRef<str>>(&mut self, token: String) {
        self.push_str(token.as_ref());
        self.push(' ');
    }
}

impl BindingGenerator {
    /// function to create a new BindingGenerator
    pub fn new(attr: &Attributes) -> Self {
        let mut sized_integer = false;
        if attr.attributes.iter().any(|attribute| {
            if let Attribute::Named(ident, lit) = attribute {
                (ident.name.as_str(), lit.to_string().as_str()) == ("integer", "sized")
            } else {
                  false
            }
        }) {
            sized_integer = true;
        }

        Self { sized_integer }
    }

    pub fn generate_prelude(&self, context: &Context) -> String {
        let mut content = String::new();
        content.push_line("#pragma once");
        content.push_line("#include <stdint.h>");
        content.push_line("#ifdef __cplusplus\n");
        content.push_line("extern \"C\" {");
        content.push_line("#endif\n");
        content
    }

    pub fn generate_type(&self, context: &Context, implementation: &Implementation) -> String {
        let mut content = String::new();
        content.push_line(format!("typedef struct Struct_{} {{", implementation.self_.name));
        content.push_line("void* self;");
        content.push_line(format!("}} {};", implementation.self_.name));
        content
    }

    pub fn generate_output(&self, _context: &Context, output: &Option<ir::Type>) -> String {
        let type_ = output
            .map(|type_| Type::from(type_.clone()).to_string())
            .unwrap_or_else(|| "void".into());
        format!("{} ", type_)
    }

    pub fn generate_method(&self, context: &Context, implementation: &Implementation, method: &Function) -> String {
        let mut content = self.generate_output(context, &method.output);
        content.push_line(self.generate_function_name(context, implementation, method));
        // FIXME: This naming convention happens in the extern generator and here. How can we generalize this code?
        let name = format!("{}_{}", &implementation.self_.name, &method.identifier.name);

        // method
        //     .inputs
        //     .iter()
        //     .map(|parameter| {
        //
        //     })
        //     .collect_vec()
        //     .join(',');
        // {
        //     let type_ = Type::from(parameter.type_.clone());
        //     let ident = &parameter.identifier.name;
        //     content.push_str(format!("{} {}", type_, ident));
        // }

        //TODO: Distinguish sized types

        content.push_line(&format!(
            "{} {}({});",
            if let Some(typ) = &method.output {
                format!("{}", Type::from(typ.clone()))
            } else {
                String::from("void")
            },
            &name,
            parameters.join(", ")
        ));
        content
    }

    pub fn generate_epilogue(&self, _context: &Context) -> String {
        let mut content = String::new();
        content.push_line("#ifdef __cplusplus");
        content.push_line("}");
        content.push_line("#endif");
        content
    }

    /// generate function for the BindingGenerator
    pub fn generate(&self, context: &Context, implementation: &Implementation) {
        let mut content = self.generate_prelude(context);
        content.push_line(self.generate_type(context, implementation));

        for item in &implementation.items {
            match item {
                Constant(_) => Logger::log("Const extern not supported."),
                Method(method) => content.push_line(self.generate_method(context, implementation, method))
            }
        }

        content.push_line(self.generate_epilogue(context));

        let header_path = context
            .arguments
            .target_dir
            .join("ligen")
            .join(&context.arguments.crate_name)
            .join("include")
            .join(format!("{}.h", implementation.self_.name));

        let mut file = File::create(&header_path)
            .expect(&format!("Failed to create file {}.", header_path.display()));
        file.write_all(content.as_bytes()).expect("Couldn't write file.");
    }
}
