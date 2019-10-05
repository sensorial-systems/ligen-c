use crate::ast::{FunctionStatement};
use std::fmt;

pub struct FunctionBlock {
    function_statements : Vec<FunctionStatement>
}

impl FunctionBlock {
    pub fn new(function_statements : Vec<FunctionStatement>) -> Self {
        Self {
            function_statements
        }
    }
}

impl fmt::Display for FunctionBlock {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for statement in &self.function_statements {
            write!(f, "\n\t{};", statement)?;
        }
        write!(f, "\n}}")
    }
}
