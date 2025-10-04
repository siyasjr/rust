fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  error: cannot borrow as mutable while immutable refs exist

    println!("{} and {}", r1, r2); //  multiple immutable refs are fine
}
