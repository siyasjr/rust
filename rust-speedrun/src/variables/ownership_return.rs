fn create_message() -> String {
    let msg = String::from("Hello from John Wick!");
    msg // ownership returned
}

fn main() {
    let greeting = create_message();
    println!("{}", greeting);
}
