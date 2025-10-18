mod greetings;

fn main() {
    
    greetings::english::hello(); 

    use greetings::english::hello;
    hello();
}
