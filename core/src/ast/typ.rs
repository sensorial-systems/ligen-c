use crate::ast::Identifier;

/// Constant.
#[derive(Debug, Clone, Copy)]
pub struct Const;

/// Pointer.
#[derive(Debug, Clone, Copy)]
pub struct Pointer;

#[derive(Debug)]
/// Type Struct
pub struct Type {
    /// constness field
    constness: Option<Const>,
    /// type_ field
    type_: Identifier,
    /// pointer field
    pointer: Option<Pointer>,
}

impl Type {
    /// Function to create a new Type
    pub fn new(constness: Option<Const>, type_: Identifier, pointer: Option<Pointer>) -> Type {
        Type {
            constness,
            type_,
            pointer,
        }
    }
}

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            if let Some(_) = self.constness {
                "const "
            } else {
                ""
            },
            self.type_.name,
            if let Some(_) = self.pointer { "*" } else { "" }
        )
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Identifier;

    use super::{Const, Pointer, Type};

    #[test]
    fn ast_type_atomic() {
        let types: Vec<(Type, String)> = vec![
            "char",
            "short",
            "int",
            "long int",
            "long long int",
            "float",
            "double",
            "long double",
            "unsigned char",
            "unsigned short",
            "unsigned int",
            "unsigned long int",
            "unsigned long long int",
        ]
        .into_iter()
        .map(|typ| {
            (
                Type::new(Some(Const), Identifier::new(typ), Some(Pointer)),
                format!("const {}*", typ),
            )
        })
        .collect();

        let mut iter = types.into_iter();

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(format!("{}", value), *expected_value);
        }
    }
}
