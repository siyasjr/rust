fn main() {
    let numbers = [3, 7, 2, 9, 5];
    let mut max = numbers[0];

    for &num in numbers.iter() {
        if num > max {
            max = num;
        }
    }

    println!("The maximum number is {}", max);
}
