use crate::ast::Identifier;

pub struct Macro {
    pub identifier : Identifier,
    pub content : String
}

impl Macro {
    pub fn new(identifier: Identifier, content: &str) -> Macro {
        let content = String::from(content);
        Macro {
            identifier,
            content
        }
    }
}

use std::fmt;
impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.identifier, self.content)
    }
}
