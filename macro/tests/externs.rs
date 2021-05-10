use ligen_c::ligen_c;

pub struct Struct {}

#[ligen_c]
impl Struct {
    pub fn say() -> String {
        "Hello from Ligen!".into()
    }
}

// Should be generated from ligen_c:
// #[no_mangle]
// pub extern fn Struct_say() -> String {
//     Struct::say()
// }

#[test]
fn test() {
    assert_eq!(Struct_say(), Struct::say());
}