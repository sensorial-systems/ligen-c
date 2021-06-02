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

ligen_c_package!(cmake);