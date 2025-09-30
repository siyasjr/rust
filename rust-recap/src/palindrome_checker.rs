fn main() {
    let word = "madam";
    let reversed: String = word.chars().rev().collect();

    if word == reversed {
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }
}
