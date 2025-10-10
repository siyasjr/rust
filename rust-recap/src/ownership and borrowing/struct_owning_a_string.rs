struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User {
        name: String::from("Anas"),
        age: 22,
    };

    println!("{} is {} years old", user.name, user.age);
}
