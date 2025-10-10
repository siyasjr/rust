fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice borrowed, not owned
        }
    }

    &s[..]
}

fn main() {
    let sentence = String::from("hello rust world");
    let word = first_word(&sentence); // borrow, no move
    println!("First word: {}", word);
}
