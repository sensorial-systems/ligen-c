use crate::ast::{Function, Identifier};
use crate::generators::{TypeGenerator, ParametersGenerator};

pub struct FunctionGenerator {}

impl FunctionGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> Function {
        let return_type = if let Some(typ) = &method.output.typ { TypeGenerator::generate(&typ, sized_integer) } else { TypeGenerator::void() };
        let identifier = Identifier::new(&method.get_extern_name());
        let parameters = ParametersGenerator::generate(&method.inputs, sized_integer);
        Function::new(return_type, identifier, parameters)
    }
}
