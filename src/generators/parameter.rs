use crate::ast::{Type, Identifier, Parameter, Parameters};
use crate::generators::{TypeGenerator};

pub struct ParameterGenerator {}

impl ParameterGenerator {
    pub fn generate(input : &ligen_core::Input, sized_integer : bool) -> Parameter {
        Parameter::new(TypeGenerator::generate(&input.typ, sized_integer), Identifier::new(&input.identifier.name))
    }
}

pub struct ParametersGenerator {}

impl ParametersGenerator {
    pub fn generate(inputs : &ligen_core::Inputs, sized_integer : bool) -> Parameters {
        let mut parameters = Vec::new();
        if let Some(self_type) = &inputs.self_type {
            let is_mutable = match &self_type.modifier {
                ligen_core::TypeModifier::Reference(reference) => reference.is_mutable,
                ligen_core::TypeModifier::Pointer(reference) => reference.is_mutable,
                ligen_core::TypeModifier::None => false
            };
            let typ = Type::new(!is_mutable, Identifier::new(&self_type.identifier.name), false);
            let identifier = Identifier::new("self");
            parameters.push(Parameter::new(typ, identifier));
        }
        for input in &inputs.inputs {
            parameters.push(ParameterGenerator::generate(&input, sized_integer));
        }
        Parameters::new(parameters)
    }
}
