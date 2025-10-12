fn main() {
    let s = String::from("Hello, borrow!");
    print_length(&s);
    println!("Still own it: {}", s);
}

fn print_length(text: &String) {
    println!("Length is: {}", text.len());
}
