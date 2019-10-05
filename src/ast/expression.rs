use crate::ast::{FunctionCall, Identifier};

pub enum Expression {
    FunctionCall(FunctionCall),
    Identifier(Identifier)
}

use std::fmt;
impl fmt::Display for Expression {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::FunctionCall(value) => write!(f, "{}", value),
            Expression::Identifier(value) => write!(f, "{}", value)
        }
    }
}
