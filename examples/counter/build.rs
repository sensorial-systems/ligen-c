use ligen::ir::{Module, Attributes};
use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;
use ligen::Generator;
use ligen_c::generator::Generator as CGenerator;
use ligen_cmake::Generator as CMakeGenerator;
use ligen::generator::{Context, Arguments, SourceFile};

fn main() {
    let module = Module::root().expect("Couldn't get root module.");
    let context = Context {
        arguments: Arguments::from_env().expect("Couldn't get variables fro env."),
        source_file: SourceFile::default()
    };
    let attributes = Attributes::default();
    let c_generator = CGenerator::new(&context, &attributes);
    let cmake_generator = CMakeGenerator::new(&context, &attributes);
    cmake_generator.generate(&context, None).expect("Couldn't generate CMake");
    for object in module.objects {
        c_generator.generate(&context, Some(&object)).expect("Couldn't generate object.");
    }
}
