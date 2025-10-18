struct User {
    username: String,
    email: String,
}

fn main() {
    let user1 = User {
        username: String::from("anasz"),
        email: String::from("anas@example.com"),
    };

    let user2 = User {
        username: String::from("rustacean"),
        ..user1
    };

    // println!("{}", user1.email); ❌ moved when creating user2
    println!("{}", user2.email);
}
