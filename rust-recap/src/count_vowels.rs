fn main() {
    let text = "hello rustaceans";
    let mut count = 0;

    for c in text.chars() {
        if "aeiou".contains(c) {
            count += 1;
        }
    }

    println!("Number of vowels in '{}' is {}", text, count);
}
