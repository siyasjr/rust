fn main() {
    let x = 10; // i32 implements Copy
    let y = x;  // copies value
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("Rust");
    let s2 = s1; // moves ownership
    // println!("{}", s1); ❌ s1 moved
    println!("s2 = {}", s2);
}
