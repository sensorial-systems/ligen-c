//! C Binding Generator.

#![feature(proc_macro_span)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use ligen_c_core::{Context, SourceFile};
use proc_macro::TokenStream;

struct SF {
    source_file: SourceFile,
}

impl From<proc_macro::SourceFile> for SF {
    fn from(sf: proc_macro::SourceFile) -> Self {
        Self {
            source_file: SourceFile {
                is_real: sf.is_real(),
                path: sf.path(),
            },
        }
    }
}

/// Entry point for ligen_c
#[proc_macro_attribute]
pub fn ligen_c(args: TokenStream, input: TokenStream) -> TokenStream {
    ligen_c_core::ligen_c(
        Context {
            source_file: SF::from(proc_macro::Span::call_site().source_file()).source_file,
        },
        args.into(),
        input.into(),
    )
    .into()
}
