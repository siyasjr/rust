fn print_len(s: &String) {
    println!("Length = {}", s.len());
}

fn main() {
    let s = String::from("rust");
    print_len(&s); // pass as reference
    println!("Still valid: {}", s); // ✅ no ownership lost
}
