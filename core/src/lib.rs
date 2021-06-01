//! C Binding Generator.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

pub mod ast;
pub mod generator;

mod proc_macro;
pub use ligen_core::proc_macro::{Context, SourceFile};
pub use proc_macro::{ligen_c, ligen_c_package};
