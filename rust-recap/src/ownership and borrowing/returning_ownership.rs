fn takes_and_returns(s: String) -> String {
    println!("Got: {}", s);
    s // ownership returned
}

fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_returns(s1); // s1 moved, s2 takes it back
    println!("Returned: {}", s2);
}
