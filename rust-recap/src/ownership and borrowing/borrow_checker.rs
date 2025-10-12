fn main() {
    let mut data = String::from("Rust");
    let ref1 = &data;
    println!("Read: {}", ref1);

    let ref2 = &mut data; // ❌ Cannot borrow mutably while immutable ref exists
    ref2.push_str(" language");
}
