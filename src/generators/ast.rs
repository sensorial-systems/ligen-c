use crate::ast::{AST, Statement, Macro, Identifier, Struct, TypeDef, VariableDecl, Type};
use crate::generators::{FunctionGenerator};

pub struct ASTGenerator {}

impl ASTGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> AST {
        let mut statements = Vec::new();
        statements.push(Statement::Macro(Macro::new(Identifier::new("pragma"), "once")));
        if sized_integer { statements.push(Statement::Macro(Macro::new(Identifier::new("include"), "<stdint.h>"))); }
        statements.push(Statement::Macro(Macro::new(Identifier::new("include"), "<stdbool.h>")));
        for dependency in &object.dependencies { statements.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("<{}.h>", dependency.name)))); }
        statements.push(Statement::Macro(Macro::new(Identifier::new("ifdef"), "__cplusplus")));
        statements.push(Statement::Uncategorized(String::from("extern \"C\" {")));
        statements.push(Statement::Macro(Macro::new(Identifier::new("endif"), "")));
        statements.push(Statement::Struct(Struct::new(Identifier::new(&object.typ.identifier.name), vec![VariableDecl::new(Type::new(false, Identifier::new("void"), true), Identifier::new("self"))])));
        statements.push(Statement::TypeDef(TypeDef::new(Identifier::new(&format!("struct {}", object.typ.identifier.name)), Identifier::new(&object.typ.identifier.name))));
        for method in &object.methods { statements.push(Statement::Function(FunctionGenerator::generate(&method, sized_integer))) };
        statements.push(Statement::Macro(Macro::new(Identifier::new("ifdef"), "__cplusplus")));
        statements.push(Statement::Uncategorized(String::from("}")));
        statements.push(Statement::Macro(Macro::new(Identifier::new("endif"), "")));

        AST::new(statements)
    }
}
