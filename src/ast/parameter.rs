use crate::ast::{Type, Identifier};

pub struct Parameter {
    pub typ : Type,
    pub identifier : Identifier
}

impl Parameter {
    pub fn new(typ : Type, identifier : Identifier) -> Parameter {
        Parameter {
            typ,
            identifier
        }
    }
}

pub struct Parameters {
    pub parameters : Vec<Parameter>
}

impl Parameters {
    pub fn new(parameters : Vec<Parameter>) -> Self {
        Self {
            parameters
        }
    }
}

use std::fmt;
impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.typ, self.identifier)
    }
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, parameter) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", parameter)?;
        }
        write!(f, "")
    }
}
