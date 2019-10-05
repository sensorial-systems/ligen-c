use crate::ast::{Identifier, Expression};

pub struct Assignment {
    pub lvalue : Identifier,
    pub rvalue : Expression
}

impl Assignment {
    pub fn new(lvalue : Identifier, rvalue : Expression) -> Self {
        Self {
            lvalue,
            rvalue
        }
    }
}

use std::fmt;
impl fmt::Display for Assignment {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.lvalue, self.rvalue)
    }
}
