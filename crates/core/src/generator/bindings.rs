use crate::ast::{Type, Types};

use ligen::ir::Implementation;
use ligen::ir;
use ir::Visibility;
use ir::Attribute;
use ir::Attributes;
use ir::Function;
use ligen::generator::Context;

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

// FIXME: Needs better API / generalization.
/// Extension for adding source code text to a string.
pub trait StringExt {
    /// Push a new line.
    fn push_line<String: AsRef<str>>(&mut self, line: String);
    /// Push a text followed by a space character.
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

    /// Generate source file prelude.
    pub fn generate_prelude(&self, _context: &Context) -> String {
        let mut content = String::new();
        content.push_line("#pragma once");
        content.push_line("");
        content.push_line("#include <stdint.h>");
        content.push_line("");
        content.push_line("#ifdef __cplusplus");
        content.push_line("extern \"C\" {");
        content.push_line("#endif\n");
        content
    }

    /// Generate struct.
    pub fn generate_struct(&self, _context: &Context, implementation: &Implementation) -> String {
        let mut content = String::new();
        content.push_line(format!(
            "typedef struct Struct_{} {{",
            implementation.self_.name
        ));
        content.push_line("void* self;");
        content.push_line(format!("}} {};", implementation.self_.name));
        content
    }

    /// Generate function output.
    pub fn generate_function_output(&self, _context: &Context, output: &Option<ir::Type>) -> String {
        let type_ = output
            .as_ref()
            .map(|type_| {
                let typ_ = Type::from(type_.clone());
                if let Types::Compound(compound) = typ_.type_ {
                    match compound.name.as_str() {
                        "String" => "RString".to_string(),
                        _ => Type::from(type_.clone()).to_string(),
                    }
                } else {
                    Type::from(type_.clone()).to_string()
                }
            })
            .unwrap_or_else(|| "void".into());
        format!("{} ", type_)
    }

    /// Generate function name.
    pub fn generate_function_name(
        &self,
        _context: &Context,
        implementation: &Implementation,
        method: &Function,
    ) -> String {
        // FIXME: This naming convention happens in the extern generator and here. How can we generalize this code?
        format!("{}_{}", &implementation.self_.name, &method.identifier.name)
    }

    /// Generate function parameter.
    pub fn generate_function_parameter(
        &self,
        _context: &Context,
        parameter: &ir::Parameter,
    ) -> String {
        let mut type_ = Type::from(parameter.type_.clone());
        if let (Some(_pointer), Types::Compound(_type)) = (&type_.pointer, &type_.type_) {
            type_.pointer = None;
        }
        let ident = &parameter.identifier.name;
        format!("{} {}", type_, ident)
    }

    /// Generate function parameters.
    pub fn generate_function_parameters(&self, context: &Context, method: &Function) -> String {
        method
            .inputs
            .iter()
            .map(|parameter| self.generate_function_parameter(context, parameter))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Generate function.
    pub fn generate_function(
        &self,
        context: &Context,
        implementation: &Implementation,
        method: &Function,
    ) -> String {
        let mut content = String::new();
        if let Visibility::Public = method.visibility {
            content.push_str(&self.generate_function_output(context, &method.output));
            content.push_str(&self.generate_function_name(context, implementation, method));
            content.push('(');
            content.push_str(&self.generate_function_parameters(context, method));
            content.push_str(");");
        }
        content
    }

    /// Generate source file epilogue.
    pub fn generate_epilogue(&self, _context: &Context) -> String {
        let mut content = String::new();
        content.push_line("");
        content.push_line("#ifdef __cplusplus");
        content.push_line("}");
        content.push_line("#endif");
        content
    }

    /// Generate drop binding.
    pub fn generate_drop(object_name: &String) -> String {
        format!(
            "void {0}_drop({0} {1});",
            object_name,
            object_name.to_lowercase()
        )
    }
}
