fn main() {
    let text = "banana";
    let mut count = 0;

    for c in text.chars() {
        if c == 'a' {
            count += 1;
        }
    }

    println!("'a' appears {} times in '{}'", count, text);
}
