fn main() {
    let arr = [12, 5, 7, 3, 9];
    let mut min = arr[0];

    for &num in arr.iter() {
        if num < min {
            min = num;
        }
    }

    println!("Minimum value is {}", min);
}
