struct Person {
    name: String,
}

fn main() {
    let p1 = Person {
        name: String::from("Anas"),
    };

    let p2 = p1; // ownership moved
    // println!("{}", p1.name); ❌ Error: p1 moved
    println!("Person name: {}", p2.name);
}
