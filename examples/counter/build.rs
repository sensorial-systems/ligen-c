use ligen::ir::Project;
use ligen::Generator;
use ligen_c::generator::Generator as CGenerator;
use ligen_cmake::Generator as CMakeGenerator;

fn main() {
    // Project binding generation.
    if let Ok(project) = Project::read() {
        // Generators.
        let c_generator = CGenerator::default();
        let cmake_generator = CMakeGenerator::default();
        cmake_generator.generate(&project).expect("Couldn't generate CMake project.");
        c_generator.generate(&project).expect("Couldn't generate C bindings");
    }
}
