#[derive(Clone)]
pub struct Identifier {
    pub name : String
}

impl Identifier {
    pub fn new(name : &str) -> Identifier {
        Identifier {
            name : String::from(name)
        }
    }
}

use std::fmt;
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
