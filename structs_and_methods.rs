use std::io;
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn create_rect(width:u32, height: u32) -> Rectangle {
        Rectangle{width: width, height: height}
    }
}

fn main() {

    println!("Create a rectangle");
    println!("Please enter the width:");
    
    let mut new_rect_width = String::new();
    
    io::stdin().read_line(&mut new_rect_width)
    .expect("Failed to read");
    
    println!("Please enter the height:");
    
    let mut new_rect_height = String::new();
    
    io::stdin().read_line(&mut new_rect_height)
    .expect("Failed to read");

    let new_rect_width:u32 = match new_rect_width.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
     let new_rect_height:u32 = match new_rect_height.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    let new_rect = Rectangle::create_rect(new_rect_width, new_rect_height);
    
    println!("The area of your rectangle is {}", new_rect.area());
    
    println!("{:?}", new_rect);
    
}
