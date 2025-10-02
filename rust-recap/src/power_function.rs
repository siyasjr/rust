fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}

fn main() {
    let base = 3;
    let exp = 4;
    println!("{}^{} = {}", base, exp, power(base, exp));
}
