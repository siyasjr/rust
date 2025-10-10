struct UserProfile<'a> {
    username: &'a str, // borrow, doesn’t own
}

fn main() {
    let name = String::from("Anas");
    let profile = UserProfile { username: &name };

    println!("Username: {}", profile.username);
}
