fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let mut sum = 0;

    for &n in numbers.iter() {
        sum += n;
    }

    let avg = sum as f64 / numbers.len() as f64;
    println!("Average = {}", avg);
}
