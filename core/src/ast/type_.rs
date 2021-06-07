use ligen_core::ir::Integer;
use ligen_core::ir;

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
    pub constness: Option<Const>,
    /// type_ field
    pub type_: Types,
    /// pointer field
    pub pointer: Option<Pointer>,
}

impl Type {
    /// Function to create a new Type
    pub fn new(constness: Option<Const>, type_: Types, pointer: Option<Pointer>) -> Type {
        Type {
            constness,
            type_,
            pointer,
        }
    }
}

impl From<ligen_core::ir::Atomic> for Atomic {
    fn from(atomic: ligen_core::ir::Atomic) -> Self {
        match atomic {
            ligen_core::ir::Atomic::Integer(integer) => match integer {
                Integer::U8 => Atomic::UnsignedChar,
                Integer::U16 => Atomic::UnsignedShort,
                Integer::U32 => Atomic::UnsignedInt,
                Integer::U64 => Atomic::UnsignedLongLongInt,
                Integer::I8 => Atomic::Char,
                Integer::I16 => Atomic::Short,
                Integer::I32 => Atomic::Int,
                Integer::I64 => Atomic::LongLongInt,
                Integer::U128 | Integer::USize | Integer::I128 | Integer::ISize => {
                    panic!("Atomic types u128, usize, i128 and isize not implemented")
                }
            },
            ligen_core::ir::Atomic::Float(float) => match float {
                ligen_core::ir::Float::F32 => Atomic::Float,
                ligen_core::ir::Float::F64 => Atomic::Double,
            },
            ligen_core::ir::Atomic::Boolean => panic!("Boolean not implemented"),
            ligen_core::ir::Atomic::Character => panic!("16bit char not implemented"),
        }
    }
}

impl From<ligen_core::ir::Type> for Types {
    fn from(typ: ligen_core::ir::Type) -> Self {
        match typ {
            ligen_core::ir::Type::Atomic(atomic) => Self::Atomic(Atomic::from(atomic)),
            ligen_core::ir::Type::Compound(compound) => {
                Self::Compound(Identifier::new(&compound.name))
            }
            ligen_core::ir::Type::Reference(reference) => Self::from(match reference {
                ligen_core::ir::Reference::Borrow(borrow) => match borrow {
                    ligen_core::ir::Borrow::Constant(constant) => Self::from(*constant),
                    ligen_core::ir::Borrow::Mutable(mutable) => Self::from(*mutable),
                },
                ligen_core::ir::Reference::Pointer(pointer) => match pointer {
                    ligen_core::ir::Pointer::Constant(constant) => Self::from(*constant),
                    ligen_core::ir::Pointer::Mutable(mutable) => Self::from(*mutable),
                },
            }),
        }
    }
}

impl From<ligen_core::ir::Type> for Type {
    fn from(typ: ligen_core::ir::Type) -> Self {
        match typ {
            ligen_core::ir::Type::Atomic(_) | ligen_core::ir::Type::Compound(_) => Self {
                constness: None,
                type_: Types::from(typ),
                pointer: None,
            },
            ir::Type::Reference(reference) => {
                // FIXME: This needs to be simplified.
                match reference {
                    ir::Reference::Pointer(pointer) => {
                        match pointer {
                            ir::Pointer::Constant(constant) => Self {
                                constness: Some(Const),
                                type_: Types::from(*constant),
                                pointer: Some(Pointer),
                            },
                            ir::Pointer::Mutable(mutable) => Self {
                                constness: None,
                                type_: Types::from(*mutable),
                                pointer: Some(Pointer),
                            },
                        }
                    },
                    ir::Reference::Borrow(borrow) => {
                        match borrow {
                            ir::Borrow::Constant(constant) => Self {
                                constness: Some(Const),
                                type_: Types::from(*constant),
                                pointer: Some(Pointer),
                            },
                            ir::Borrow::Mutable(mutable) => Self {
                                constness: None,
                                type_: Types::from(*mutable),
                                pointer: Some(Pointer)
                            }
                        }
                    }
                }
            }
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
            if let Some(_) = self.pointer { "*" } else { "" }
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
