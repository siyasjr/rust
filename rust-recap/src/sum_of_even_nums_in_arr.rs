fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let mut sum = 0;

    for &num in arr.iter() {
        if num % 2 == 0 {
            sum += num;
        }
    }

    println!("Sum of even numbers is {}", sum);
}
