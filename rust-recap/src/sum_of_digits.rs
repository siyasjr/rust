fn main() {
    let mut n = 12345;
    let mut sum = 0;

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    println!("Sum of digits is {}", sum);
}
