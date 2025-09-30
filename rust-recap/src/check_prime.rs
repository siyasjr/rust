fn main() {
    let n = 29;
    let mut is_prime = true;

    if n <= 1 {
        is_prime = false;
    } else {
        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
    }

    if is_prime {
        println!("{} is prime", n);
    } else {
        println!("{} is not prime", n);
    }
}
