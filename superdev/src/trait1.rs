

struct Rect{
    height: u32,
    width: u32,
}


struct Square{
    side: u32,
}

impl Rect{
    fn calc_area(&self) -> u32{
        self.height * self.width
    }

    fn name()-> String{
        return String::from("Recatangle");
    }
}


impl Square{
    fn calc_area(&self) -> u32{
        self.side * self.side
    }

    fn name()->String{
        return String::from("Square");
    }
}

fn main(){

    let rec = Rect{
        height: 32,
        width: 40,
    };
let squ = Square{
        side: 45,
    };


    println!("{},{},{},{}", rec.calc_area(), squ.calc_area(), Rect::name(),Square::name());

}
