fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn change_name(name: &mut String) {
    name.push_str(" Wick");
}

fn main() {
    let mut name = String::from("John");
    greet(&name);
    change_name(&mut name);
    greet(&name);
}
