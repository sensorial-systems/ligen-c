use ligen::generator::{ImplementationVisitor, FileProcessorVisitor, Context, FileSet, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors};
use ligen::ir;
use std::path::PathBuf;
use crate::ast::{Types, Type};
use crate::generator::Generator;

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

fn path(implementation: &ImplementationVisitor) -> PathBuf {
    PathBuf::from("include").join(format!("{}.h", implementation.current.self_.path().last().name))
}

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        // includes
        file.writeln("#pragma once");
        file.writeln("");
        file.writeln("#include <stdint.h>");
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("extern \"C\" {");
        file.writeln("#endif\n");

        // structure
        file.writeln(format!("typedef struct Struct_{} {{", visitor.current.self_.path().last().name));
        file.writeln("\tvoid* self;");
        file.writeln(format!("}} {};", Type::from(visitor.current.self_.clone())));
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));

        // drop function
        let object_name = &visitor.current.self_.path().last().name;
        let c_type      = Type::from(visitor.current.self_.clone());
        file.writeln(format!("void {name}_drop({type_} self);", name = object_name, type_ = c_type));

        // epilogue
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("}");
        file.writeln("#endif");
    }
}

impl FunctionProcessor {
    /// Generate function name.
    pub fn generate_function_name(&self, visitor: &FunctionVisitor) -> String {
        // FIXME: This naming convention happens in the extern generator and here. How can we generalize this code?
        format!("{}_{}", &visitor.parent.current.self_.path().last().name, &visitor.current.identifier.name)
    }

    /// Generate function output.
    pub fn generate_function_output(&self, output: &Option<ir::Type>) -> String {
        let type_ = output
            .as_ref()
            .map(|type_| {
                let typ_ = Type::from(type_.clone());
                if let Types::Compound(compound) = typ_.type_ {
                    match compound.name.as_str() {
                        // FIXME: C prefix should be generalized like in Type::From
                        "String" => "CRString".to_string(),
                        _ => Type::from(type_.clone()).to_string(),
                    }
                } else {
                    Type::from(type_.clone()).to_string()
                }
            })
            .unwrap_or_else(|| "void".into());
        format!("{} ", type_)
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent));
            file.write(self.generate_function_output(&visitor.current.output));
            file.write(self.generate_function_name(&visitor));
            file.write("(");
        }
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent));
            file.writeln(");");
        }
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent.parent));

        let mut type_ = Type::from(visitor.current.type_.clone());
        if let (Some(_pointer), Types::Compound(_type)) = (&type_.pointer, &type_.type_) {
            type_.pointer = None;
        }
        let ident = &visitor.current.identifier.name;
        file.write(format!("{} {}", type_, ident))
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent.parent));
        file.write(", ");
    }
}

impl FileGeneratorVisitors for Generator {
    type ImplementationProcessor = ImplementationProcessor;
    type FunctionProcessor = FunctionProcessor;
    type ParameterProcessor = ParameterProcessor;
}
