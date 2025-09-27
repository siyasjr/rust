
fn main() {

// print_name();

// is_even();

sum();
    

 


}





fn print_name() {
    let name: &str = "Nvidia";
    let mut age: u32 = 25;


    println!("The name is {}", name);
    println!("The age is {}", age);

    age += 1 ;

    println!("Your age after this year is {}", age);
}


fn is_even(){
    let num: i32 = 5165;
    if num % 2 == 0 {
        println!("{} is even", num);
    }
    else {
        println!("{} is odd", num)
    }
}


fn sum(){
    let mut sum: i32 = 0;

    for i in 1..=10 {
        
        sum += i;

    }

    println!("{}", sum);

}
