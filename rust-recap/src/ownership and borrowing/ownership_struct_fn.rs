struct Rectangle {
    width: u32,
    height: u32,
}

fn build_rect(w: u32, h: u32) -> Rectangle {
    Rectangle { width: w, height: h } // ownership of fields moves into struct
}

fn main() {
    let rect = build_rect(10, 20);
    println!("Area = {}", rect.width * rect.height);
}
