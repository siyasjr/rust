use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("You passed: {:?}", &args[1..]);
    } else {
        println!("No arguments passed!");
    }
}
