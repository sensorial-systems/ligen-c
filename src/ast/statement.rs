use crate::ast::{Macro, Function, Struct, TypeDef};

pub enum Statement {
    Macro(Macro),
    Function(Function),
    Struct(Struct),
    TypeDef(TypeDef),
    Uncategorized(String)
}

use std::fmt;
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Macro(value) => write!(f, "{}", value),
            Statement::Function(value) => write!(f, "{}", value),
            Statement::Struct(value) => write!(f, "{}", value),
            Statement::TypeDef(value) => write!(f, "{}", value),
            Statement::Uncategorized(value) => write!(f, "{}", value),
        }
    }
}
