use ligen::ligen;
use ligen_core::{Object, Files, File, Attributes, LiteralConverter};

pub mod ast;
pub mod generators;
use generators::{ASTGenerator};

pub struct Generator {
    pub sized_integer : bool,
    pub named_objects : bool
}

#[ligen]
impl Generator {
    pub fn new(attributes: &Attributes) -> Generator {
        let sized_integer = attributes.get_named("sized_integer").as_bool(true);
        let named_objects = attributes.get_named("named_objects").as_bool(false);

        Generator {
            sized_integer,
            named_objects
        }
    }

    pub fn generate(&self, object: &Object) -> Files {
        let path = format!("c/include/{}.h", object.typ.identifier.name);
        let ast = ASTGenerator::generate(&object, self.sized_integer);
        let object = File::new(path, format!("{}", ast));

        Files::from(vec![object])
    }
}
