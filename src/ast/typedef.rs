use crate::ast::{Identifier};

pub struct TypeDef {
    pub identifier : Identifier,
    pub new_identifier : Identifier
}

impl TypeDef {
    pub fn new(identifier : Identifier, new_identifier : Identifier) -> Self {
        Self {
            identifier,
            new_identifier
        }
    }
}

use std::fmt;
impl fmt::Display for TypeDef {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "typedef {} {};", self.identifier, self.new_identifier)
    }
}
