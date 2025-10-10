enum Message<'a> {
    Owned(String),
    Borrowed(&'a str),
}

fn main() {
    let owned_msg = Message::Owned(String::from("hello"));
    let borrowed_msg = Message::Borrowed("world");

    match owned_msg {
        Message::Owned(text) => println!("Owned: {}", text),
        Message::Borrowed(_) => (),
    }

    if let Message::Borrowed(s) = borrowed_msg {
        println!("Borrowed: {}", s);
    }
}
