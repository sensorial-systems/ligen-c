use crate::ast::{Type, Identifier, Parameters};

pub struct Function {
    pub return_type : Type,
    pub identifier : Identifier,
    pub parameters : Parameters
}

impl Function {
    pub fn new(return_type : Type, identifier : Identifier, parameters : Parameters) -> Function {
        Function {
            return_type,
            identifier,
            parameters
        }
    }
}

use std::fmt;
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}({});", self.return_type, self.identifier, self.parameters)
    }
}
