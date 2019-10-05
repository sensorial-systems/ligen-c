use crate::ast::{Args, Identifier};

pub struct ArgsGenerator {}

impl ArgsGenerator {
    pub fn generate(inputs : &ligen_core::Inputs) -> Args {
        let mut args = Vec::new();
        for input in &inputs.inputs {
            args.push(Identifier::new(&input.identifier.name));
        }
        Args::new(args)
    }
}
