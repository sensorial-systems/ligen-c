/// Identifier structure
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    /// Name field of Identifier
    pub name: String,
}

impl Identifier {
    /// Create a new Identifier
    pub fn new(name: &str) -> Self {
        let name = String::from(name);
        Self { name }
    }
}

use std::fmt;
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;

    #[test]
    fn ast_identifier() {
        let ident = Identifier::new("test");
        assert_eq!("test", format!("{}", ident));
    }
}
