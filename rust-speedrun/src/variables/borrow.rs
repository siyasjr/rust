fn add_numbers(vec: &mut Vec<i32>) {
    vec.push(10);
    vec.push(20);
}

fn main() {
    let mut numbers = vec![1, 2, 3];
    add_numbers(&mut numbers);
    println!("{:?}", numbers);
}
