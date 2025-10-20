fn take_vec(v: Vec<i32>) {
    println!("Vector inside function: {:?}", v);
}

fn main() {
    let numbers = vec![1, 2, 3];
    take_vec(numbers);
    // println!("{:?}", numbers); ❌ moved
}
