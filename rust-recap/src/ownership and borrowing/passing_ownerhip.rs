fn main() {
    let s = String::from("Rust");
    takes_ownership(s);
    // println!("{}", s); // ❌ Error: s was moved
}

fn takes_ownership(some_string: String) {
    println!("String inside function: {}", some_string);
}
