//! C Binding Generator.

ligen::define_project_generator!(name = "ligen_c_package", generator = "ligen_c_core::generator::ProjectGenerator");
ligen::define_binding_generator!(name = "ligen_c", generator = "ligen_c_core::generator::Generator");
