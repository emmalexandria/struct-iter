use std::fmt::Debug;
use std::iter::IntoIterator;
use std::vec::IntoIter;
use struct_iter::StructIter;

#[derive(Debug, Clone, StructIter)]
#[iter(trait = "std::fmt::Debug")]
struct Test {
    field: String,
    field2: i32,
    field1: String,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            field: String::from("hello"),
            field2: 423,
            field1: String::from("goodbye"),
        }
    }
}

#[test]
fn test_macro() {
    let val = Test::default();
    let props: Vec<&dyn Debug> = (&val).into_iter().collect();
    assert_eq!(props.len(), 3)
}
