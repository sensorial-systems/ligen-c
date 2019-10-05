use crate::ast::{Type, Identifier};

pub struct VariableDecl {
    pub typ : Type,
    pub identifier : Identifier
}

impl VariableDecl {
    pub fn new(typ : Type, identifier : Identifier) -> Self {
        Self {
            typ,
            identifier
        }
    }
}

use std::fmt;
impl fmt::Display for VariableDecl {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {};", self.typ, self.identifier)
    }
}
