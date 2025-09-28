fn main() {
    let text = "rust";
    let reversed: String = text.chars().rev().collect();

    println!("Original: {}", text);
    println!("Reversed: {}", reversed);
}
