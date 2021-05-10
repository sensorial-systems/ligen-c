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
    /// UnsignedChar variant
    UnsignedChar,
    /// UnsignedShort variant
    UnsignedShort,
    /// UnsignedInt variant
    UnsignedInt,
    /// UnsignedLongInt variant
    UnsignedLongInt,
    /// UnsignedLongLongInt variant
    UnsignedLongLongInt,
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
    type_: Types,
    /// reference field
    reference: Option<Pointer>,
}

impl Type {
    /// Function to create a new Type
    pub fn new(constness: Option<Const>, type_: Types, reference: Option<Pointer>) -> Type {
        Type {
            constness,
            type_,
            reference,
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
                        Atomic::UnsignedChar => "unsigned char",
                        Atomic::UnsignedShort => "unsigned short",
                        Atomic::UnsignedInt => "unsigned int",
                        Atomic::UnsignedLongInt => "unsigned long int",
                        Atomic::UnsignedLongLongInt => "unsigned long long int",
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
    use super::{Atomic, Const, Pointer, Type, Types};

    #[test]
    fn ast_type_atomic() {
        let types: Vec<Type> = vec![
            Atomic::Char,
            Atomic::Short,
            Atomic::Int,
            Atomic::LongInt,
            Atomic::LongLongInt,
            Atomic::Float,
            Atomic::Double,
            Atomic::LongDouble,
            Atomic::UnsignedChar,
            Atomic::UnsignedShort,
            Atomic::UnsignedInt,
            Atomic::UnsignedLongInt,
            Atomic::UnsignedLongLongInt,
        ]
        .into_iter()
        .map(|atomic| Type::new(Some(Const), Types::Atomic(atomic), Some(Pointer)))
        .collect();

        let expected: Vec<String> = vec![
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
        .map(|ty| format!("const {}*", ty))
        .collect();

        let mut iter = types.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(format!("{}", value), *expected_value);
        }
    }
}
