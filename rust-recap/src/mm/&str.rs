fn main() {
    let greeting = String::from("Hello Rustaceans!");
    let part = &greeting[0..5]; // Slice from index 0 to 4
    println!("Slice: {}", part);
}
