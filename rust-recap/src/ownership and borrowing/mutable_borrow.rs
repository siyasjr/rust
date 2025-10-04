fn add_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    add_world(&mut s); // borrow mutably
    println!("{}", s); // ✅ "hello world"
}
