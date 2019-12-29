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
                "bool" => "bool",
                "usize" => "size_t",
                "isize" => "size_t",
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
                "bool" => "bool",
                "usize" => "unsigned int",
                "isize" => "int",
                _ => "#ERROR_GENERATING_ATOMIC_TYPE"
            }
        }
    }

    pub fn void() -> Type {
        Type::new(false, Identifier::new("void"), false)
    }

    pub fn generate(typ : &ligen_core::Type, sized_integer : bool) -> Type {
        let constness = match &typ.modifier {
            ligen_core::TypeModifier::Pointer(reference) => !reference.is_mutable,
            ligen_core::TypeModifier::Reference(reference) => !reference.is_mutable && !typ.is_atomic(),
            ligen_core::TypeModifier::None => false
        };

        let pointer = match &typ.modifier {
            ligen_core::TypeModifier::Reference(_) => true,
            ligen_core::TypeModifier::Pointer(_) => true,
            ligen_core::TypeModifier::None => false
        } && typ.is_atomic();

        if typ.is_atomic() {
            let name = TypeGenerator::translate_atomic(&typ.identifier.name, sized_integer);
            let identifier = Identifier::new(name);
            Type::new(constness, identifier, pointer)
        } else {
            Type::new(constness, Identifier::new(&typ.identifier.name), pointer)
        }
    }
}
