//! C Binding Generator.

#![cfg_attr(cargo_ligen, feature(proc_macro_span))]

use ligen_core::proc_macro::prelude::*;

ligen::proc_macro_attribute!(name = "ligen_c", generator = "ligen_c_core::generator::Generator");
ligen::proc_macro!(name = "ligen_c_package", generator = "ligen_c_core::generator::ProjectGenerator");