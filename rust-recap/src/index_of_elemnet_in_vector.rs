fn main() {
    let nums = vec![10, 20, 30, 40, 50];
    let target = 30;

    let mut found = false;
    for (i, &n) in nums.iter().enumerate() {
        if n == target {
            println!("Found {} at index {}", target, i);
            found = true;
            break;
        }
    }

    if !found {
        println!("{} not found", target);
    }
}
