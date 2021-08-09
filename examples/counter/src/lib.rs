pub struct Counter {
    count: u32
}

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
    pub first_name: String,
    pub last_name: String
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
