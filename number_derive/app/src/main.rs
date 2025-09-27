use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};
use std::fmt::Error;

#[derive(SerializeNumberStruct, DeserializeNumberStruct, Debug, PartialEq)]
struct Swap {
    qty_1: i32,
    qty_2: i32,
    qty_3: i32,
}

fn main() {
    let s = Swap { qty_1: 1, qty_2: 2, qty_3: 1000 };
    let bytes = s.serialize();
    println!("serialized = {:?}", bytes);

    let s2 = Swap::deserialize(&bytes).unwrap();
    println!("deserialized = {:?}", s2);
    assert_eq!(s, s2);
}
