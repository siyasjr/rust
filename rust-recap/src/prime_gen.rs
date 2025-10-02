fn main() {
    let n = 20;

    println!("Primes up to {}:", n);
    for num in 2..=n {
        let mut is_prime = true;
        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            print!("{} ", num);
        }
    }
    println!();
}
