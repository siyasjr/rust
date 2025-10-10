fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let slice = &numbers[1..4]; // borrow part of the vector

    println!("Slice: {:?}", slice);
    println!("Original still usable: {:?}", numbers);
}
