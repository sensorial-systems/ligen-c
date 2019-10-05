use crate::ast::{Statement};

use std::fmt;

pub struct GenericAST<T : fmt::Display> {
    pub statements : Vec<T>
}

impl<T : fmt::Display> GenericAST<T> {
    pub fn new(statements : Vec<T>) -> GenericAST<T> {
        GenericAST {
            statements
        }
    }
}

impl<T : fmt::Display> fmt::Display for GenericAST<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{}\n", statement)?
        }
        write!(f, "")
    }
}

pub type AST = GenericAST<Statement>;
