use ligen_c::{ligen_c, ligen_c_package};

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

pub struct Person {
    first_name: String,
    last_name: String
}

#[ligen_c]
impl Person {
    // TODO: Transform String to RString.
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

ligen_c_package!(cmake);