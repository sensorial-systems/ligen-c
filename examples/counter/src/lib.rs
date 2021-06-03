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