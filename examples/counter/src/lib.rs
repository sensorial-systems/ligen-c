use ligen_c::{ligen_c, ligen_c_package};
use std::ffi::{CStr, CString};

pub struct Counter {
    count: u32
}

#[ligen_c]
impl Counter {
    pub fn new(count: u32) -> Self {
        Self { count }
    }

    pub fn count(&mut self, counts: u32) {
        self.count += counts;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    // TODO: String should be transformed to RString
    // pub fn append(a: String, b: String) -> String {
    //     format!("{}{}", a, b)
    // }
}

pub mod ffi {
    use std::os::raw::c_char;
    use std::ffi::CStr;

    pub struct RString(std::ffi::CString);

    // TODO: This should be moved to ligen-c so it could be always available to the user.
    #[super::ligen_c]
    impl RString {
        pub fn new(string: *const c_char) -> Self {
            unsafe {
                let string = std::ffi::CString::new(CStr::from_ptr(string).to_string_lossy().to_string()).expect("Failed to create RString.");
                Self(string)
            }
        }

        pub fn as_ptr(&self) -> *const c_char {
            self.0.as_ptr()
        }
    }

    impl From<std::string::String> for RString {
        fn from(string: std::string::String) -> Self {
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

ligen_c_package!(cmake);