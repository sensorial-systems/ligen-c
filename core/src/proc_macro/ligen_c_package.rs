//! Project generator entry point module.

use crate::generator::ProjectGenerator;
use ligen_core::ir::Attributes;
use ligen_core::proc_macro::Context;
use proc_macro2::TokenStream;
use std::convert::TryFrom;
use quote::quote;

/// Project generator entry point
pub fn ligen_c_package(context: Context, args: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Couldn't get attributes.");
    ProjectGenerator::generate(&context, args);

    // TODO: Move this to generator/ffi.rs
    quote! {
        pub mod ffi {
            use std::os::raw::c_char;

            pub struct RString(std::ffi::CString);

            #[ligen_c::ligen_c]
            impl RString {
                pub fn new(string: *const c_char) -> Self {
                    unsafe {
                        let string = std::ffi::CString::new(std::ffi::CStr::from_ptr(string).to_string_lossy().to_string()).expect("Failed to create RString.");
                        Self(string)
                    }
                }

                pub fn as_ptr(&self) -> *const c_char {
                    self.0.as_ptr()
                }
            }

            impl From<String> for RString {
                fn from(string: String) -> Self {
                    let string = std::ffi::CString::new(string).expect("Couldn't create CString.");
                    Self(string)
                }
            }

            impl From<RString> for String {
                fn from(string: RString) -> Self {
                    string.0.to_string_lossy().to_string()
                }
            }
        }
    }
}
