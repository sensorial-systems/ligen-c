use crate::ast::{VariableDecl, Assignment, Expression};

pub enum FunctionStatement {
    VariableDecl(VariableDecl),
    Assignment(Assignment),
    Return(Expression),
    Expression(Expression)
}

use std::fmt;
impl fmt::Display for FunctionStatement {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionStatement::VariableDecl(value) => write!(f, "{}", value),
            FunctionStatement::Assignment(value) => write!(f, "{}", value),
            FunctionStatement::Return(value) => write!(f, "return {}", value),
            FunctionStatement::Expression(value) => write!(f, "{}", value)
        }
    }
}
