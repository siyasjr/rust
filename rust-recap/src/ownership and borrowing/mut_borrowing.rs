fn main() {
    let mut name = String::from("Anas");
    add_lastname(&mut name);
    println!("Full name: {}", name);
}

fn add_lastname(name: &mut String) {
    name.push_str(" Zaman");
}
