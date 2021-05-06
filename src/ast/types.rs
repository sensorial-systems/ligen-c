use crate::ast::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Atomic Enum
pub enum Atomic {
    /// Char variant
    Char,
    /// Short variant
    Short,
    /// Int variant
    Int,
    /// LongInt variant
    LongInt,
    /// LongLongInt variant
    LongLongInt,
    /// Float variant
    Float,
    /// Dobule variant
    Double,
    /// LongDouble variant
    LongDouble,
}

#[derive(Debug, PartialEq)]
/// Types Enum
pub enum Types {
    /// Atomic variant
    Atomic(Atomic),
    /// Compound variant
    Compound(Identifier),
}

#[derive(Debug)]
pub struct Const;
#[derive(Debug)]
pub struct Ref;
#[derive(Debug)]
pub struct Sign;

#[derive(Debug)]
/// Type Struct
pub struct Type {
    /// constness field
    constness: Option<Const>,
    /// type_ field
    type_: Types,
    /// reference field
    reference: Option<Ref>,
    /// signed field
    signed: Option<Sign>,
}

impl Type {
    /// Function to create a new Type
    pub fn new(
        constness: Option<Const>,
        type_: Types,
        reference: Option<Ref>,
        signed: Option<Sign>,
    ) -> Type {
        Type {
            constness,
            type_,
            reference,
            signed,
        }
    }
}

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if let Some(_) = self.constness {
                "const "
            } else {
                ""
            },
            if let Some(_) = self.signed {
                "signed "
            } else {
                "unsigned "
            },
            match &self.type_ {
                Types::Atomic(atomic) => {
                    match atomic {
                        Atomic::Char => "char",
                        Atomic::Short => "short",
                        Atomic::Int => "int",
                        Atomic::LongInt => "long int",
                        Atomic::LongLongInt => "long long int",
                        Atomic::Float => "float",
                        Atomic::Double => "double",
                        Atomic::LongDouble => "long double",
                    }
                }
                Types::Compound(identifier) => identifier.name.as_str(),
            },
            if let Some(_) = self.reference {
                "*"
            } else {
                ""
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Atomic, Const, Ref, Sign, Type, Types};

    #[test]
    fn ast_type() {
        let ty = Type::new(
            Some(Const),
            Types::Atomic(Atomic::Int),
            Some(Ref),
            Some(Sign),
        );
        assert_eq!("const signed int*", format!("{}", ty));
    }
}
