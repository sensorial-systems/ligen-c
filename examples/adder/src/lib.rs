use ligen_c::ligen_c;

pub struct Adder {}

#[ligen_c]
impl Adder {
    pub fn add(a: f32, b: f32) -> f32 {
        a + b
    }
}
