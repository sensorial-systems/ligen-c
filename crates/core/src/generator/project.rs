use ligen::prelude::*;
use ligen::ir::Attributes;
use ligen::generator::{Context, FileSet, FileGenerator, Generator, FFIGenerator, ImplementationVisitor};
use ligen::generator::File;
use std::path::PathBuf;

/// CMake project generator.
#[derive(Debug, Clone)]
// FIXME: Rewrite it.
pub struct ProjectGenerator {
    attributes: Attributes
}

impl Generator for ProjectGenerator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let attributes = attributes.clone();
        Self { attributes }
    }
}

impl FileGenerator for ProjectGenerator {
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, _implementation: Option<&ImplementationVisitor>) {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &context.arguments.crate_name;

        let content = format!(
            include_str!("CMakeLists.txt"),
            generator_version = generator_version,
            project_name = project_name
        );
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.insert(file);
    }
}

impl FFIGenerator for ProjectGenerator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ImplementationVisitor>) -> TokenStream {
        quote! {
            pub mod ffi {
                use std::ffi::CStr;
                use std::os::raw::c_char;

                pub struct CChar(*const c_char);

                impl From<CChar> for String {
                    fn from(cchar: CChar) -> Self {
                        unsafe { CStr::from_ptr(cchar.0).to_str().unwrap().to_string() }
                    }
                }

                pub struct RString(std::ffi::CString);

                #[ligen_c::ligen_c]
                impl RString {
                    pub fn new(string: *const c_char) -> Self {
                        string.into()
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

                impl From<*const c_char> for RString {
                    fn from(c_char: *const c_char) -> Self {
                        unsafe {
                            let string = std::ffi::CString::new(
                                std::ffi::CStr::from_ptr(c_char)
                                    .to_string_lossy()
                                    .to_string(),
                            )
                                .expect("Failed to create RString.");
                            Self(string)
                        }
                    }
                }
            }
        }
    }
}