fn main() {
    let n = 10; // number of terms
    let (mut a, mut b) = (0, 1);

    println!("Fibonacci sequence:");
    for _ in 0..n {
        println!("{}", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
}
