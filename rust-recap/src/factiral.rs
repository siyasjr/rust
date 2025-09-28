// Factorial with a Loop

fn main() {
    let mut result = 1;
    let n = 5;

    for i in 1..=n {
        result *= i;
    }

    println!("Factorial of {} is {}", n, result);
}
