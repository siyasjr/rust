enum Data<'a> {
    Owned(String),
    Borrowed(&'a str),
}

fn main() {
    let d1 = Data::Owned(String::from("Owned data"));
    let s = "Borrowed data";
    let d2 = Data::Borrowed(s);

    match d1 {
        Data::Owned(val) => println!("Owned: {}", val),
        Data::Borrowed(val) => println!("Borrowed: {}", val),
    }

    match d2 {
        Data::Owned(val) => println!("Owned: {}", val),
        Data::Borrowed(val) => println!("Borrowed: {}", val),
    }
}
