use crate::ast::{Identifier};

pub struct Args {
    pub args : Vec<Identifier>
}

impl Args {
    pub fn new(args : Vec<Identifier>) -> Self {
        Self {
            args
        }
    }
}

use std::fmt;
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, "")
    }
}
