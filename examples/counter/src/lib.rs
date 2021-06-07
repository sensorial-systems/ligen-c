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
}

pub mod ffi {
    use std::os::raw::c_char;
    use std::ffi::CStr;

    pub struct CString(std::ffi::CString);

    #[super::ligen_c]
    impl CString {
        pub fn new(string: *const c_char) -> Self {
            unsafe {
                let string = std::ffi::CString::new(CStr::from_ptr(string).to_string_lossy().to_string()).expect("Yes");
                Self(string)
            }
        }

        pub fn as_ptr(&self) -> *const c_char {
            self.0.as_ptr()
        }
    }

    impl From<std::string::String> for CString {
        fn from(string: std::string::String) -> Self {
            let string = std::ffi::CString::new(string).expect("Couldn't create CString.");
            Self(string)
        }
    }

    impl From<CString> for String {
        fn from(string: CString) -> Self {
            string.0.to_string_lossy().to_string()
        }
    }
}

impl Counter {
    pub fn append(a: String, b: String) -> String {
        format!("{}{}", a, b)
    }
}

use std::os::raw::c_char;

#[no_mangle]
pub extern fn Counter_append(a: *const c_char, b: *const c_char) -> *const c_char {
    unsafe {
        let a = CStr::from_ptr(a).to_string_lossy().to_string();
        let b = CStr::from_ptr(b).to_string_lossy().to_string();
        let c = CString::new(Counter::append(a, b)).expect("Couldn't create CString");
        let p = c.as_ptr();
        std::mem::forget(c); //FIXME: Memory leak.
        p
    }
}

ligen_c_package!(cmake);