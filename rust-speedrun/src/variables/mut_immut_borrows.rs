fn main() {
    let data = String::from("Rustacean");
    let ref1 = &data;
    let ref2 = &data;

    println!("ref1 = {}, ref2 = {}", ref1, ref2);
    // both allowed since they are read-only
}
