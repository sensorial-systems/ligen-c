use crate::ast::{ Identifier, VariableDecl };

pub struct Struct {
    pub identifier : Identifier,
    pub variables : Vec<VariableDecl>
}

impl Struct {
    pub fn new(identifier : Identifier, variables : Vec<VariableDecl>) -> Self {
        Self {
            identifier,
            variables
        }
    }
}

use std::fmt;
impl fmt::Display for Struct {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {} {{", self.identifier)?;
        for variable in &self.variables {
            write!(f, "\n\t{}", variable)?;
        }
        write!(f, "\n}};")
    }
}
