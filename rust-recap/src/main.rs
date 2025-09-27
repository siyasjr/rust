
fn main() {

 print_name();

 


}





fn print_name() {
    let name: &str = "Nvidia";
    let mut age: u32 = 25;


    println!("The name is {}", name);
    println!("The age is {}", age);

    age += 1 ;

    println!("Your age after this year is {}", age);
}
