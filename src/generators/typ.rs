use crate::ast::{Type, Identifier};

pub struct TypeGenerator {}

impl TypeGenerator {
    pub fn translate_atomic(name: &str, sized_integer : bool) -> &str {
        if sized_integer {
            match name {
                "u64" => "uint64_t",
                "u32" => "uint32_t",
                "u16" => "uint16_t",
                "u8" => "uint8_t",
                "i64" => "int64_t",
                "i32" => "int32_t",
                "i16" => "int16_t",
                "i8" => "int8_t",
                "f32" => "float",
                "f64" => "double",
                _ => "#ERROR_GENERATING_ATOMIC_TYPE"
            }
        } else {
            match name {
                "u64" => "unsigned long",
                "u32" => "unsigned int",
                "u16" => "unsigned short",
                "u8" => "unsigned char",
                "i64" => "long",
                "i32" => "int",
                "i16" => "short",
                "i8" => "char",
                "f32" => "float",
                "f64" => "double",
                _ => "#ERROR_GENERATING_ATOMIC_TYPE"
            }
        }
    }

    pub fn void() -> Type {
        Type::new(false, Identifier::new("void"), false)
    }

    pub fn generate(ty : &ligen_core::Type, sized_integer : bool) -> Type {
        let constness = if let Some(reference) = &ty.reference {
            !reference.is_mutable && !ty.is_atomic()
        } else {
            false
        };

        if ty.is_atomic() {
            let name = TypeGenerator::translate_atomic(&ty.identifier.name, sized_integer);
            let identifier = Identifier::new(name);
            Type::new(constness, identifier, !ty.is_atomic())
        } else {
            Type::new(constness, Identifier::new(&ty.identifier.name), false)
        }
    }
}
