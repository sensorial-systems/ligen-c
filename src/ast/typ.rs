use crate::ast::Identifier;

pub struct Type {
    pub constness : bool,
    pub identifier : Identifier,
    pub reference : bool
}

impl Type {
    pub fn new(constness : bool, identifier : Identifier, reference : bool) -> Type {
        Type {
            constness,
            identifier,
            reference
        }
    }
}

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.constness {
            write!(f, "const ")?;
        }
        write!(f, "{}", self.identifier)?;
        if self.reference {
            write!(f, "*")?;
        }
        write!(f, "")
    }
}
