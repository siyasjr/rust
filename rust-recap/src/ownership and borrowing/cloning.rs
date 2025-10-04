fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy

    println!("s1 = {}, s2 = {}", s1, s2); // ✅ both valid
}
