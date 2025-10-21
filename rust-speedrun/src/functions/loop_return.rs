 pub fn looprtn(){
    let mut x: i32 = 0;

    let result = loop {
        x += 1;

        if x==5 {
            break x*2
            
        }
    };
    println!("{}", result)
 }