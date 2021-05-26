use ligen_c::ligen_c;

pub struct Struct {}

#[ligen_c]
impl Struct {
    pub fn say() -> String {
        "Hello from Ligen!".into()
    }
}

pub struct StructMultiple {}

#[ligen_c]
impl StructMultiple {
    pub fn say() -> String {
        "Hello from Ligen!".into()
    }
    pub fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    pub fn format(x: String, y: String) -> String {
        format!("x: {}, y: {}", x, y)
    }
}

// FIXME: This will never happen. How can we use cargo-ligen to test it?
#[cfg(all(test, cargo_ligen))]
mod test {
    #[test]
    fn externs() {
        assert_eq!(Struct_say(), Struct::say());
    }

    #[test]
    fn externs_multiple() {
        assert_eq!(
            (
                StructMultiple_say(),
                StructMultiple_sum(40, 2),
                StructMultiple_format("te".into(), "st".into())
            ),
            (
                StructMultiple::say(),
                StructMultiple::sum(40, 2),
                StructMultiple::format("te".into(), "st".into())
            )
        );
    }
}