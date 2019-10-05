use crate::ast::{Identifier, Args};

pub struct FunctionCall {
    pub identifier : Identifier,
    pub args : Args
}

impl FunctionCall {
    pub fn new(identifier : Identifier, args : Args) -> Self {
        Self {
            identifier,
            args
        }
    }
}

use std::fmt;
impl fmt::Display for FunctionCall {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.identifier, self.args)
    }
}
