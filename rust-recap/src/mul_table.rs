fn main() {
    let n = 7;

    println!("Multiplication table for {}", n);
    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n * i);
    }
}
