fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    for n in &nums {
        println!("{} squared is {}", n, n * n);
    }
}
