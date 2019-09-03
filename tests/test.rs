use ligen::ligen;

pub struct Rectangle {
    width: u32,
    height: u32
}

#[ligen(C)]
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
    pub fn print(&self) {
        println!("Rectangle(width: {}, height: {})", self.width, self.height);
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Dropping Rectangle");
    }
}
