fn loop_return() {
let result = loop {
    let mut x = 0;
    x += 1;
    if x == 3 {
        break x * 2; // returns 6
    }
};
println!("Result = {}", result);

}