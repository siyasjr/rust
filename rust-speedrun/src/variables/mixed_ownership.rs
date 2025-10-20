struct Article<'a> {
    title: String,
    author: &'a str,
}

fn main() {
    let author = String::from("John Wick");
    let article = Article {
        title: String::from("Learning Rust Ownership"),
        author: &author,
    };

    println!("{} by {}", article.title, article.author);
}
