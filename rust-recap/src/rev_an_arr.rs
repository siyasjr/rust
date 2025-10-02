fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let len = arr.len();

    for i in 0..len / 2 {
        arr.swap(i, len - i - 1);
    }

    println!("Reversed array: {:?}", arr);
}
