use std::io;

fn main() {

println!("Enter your name:");

let mut name = String::new();

io::stdin().read_line(&mut name)
.expect("Failed to read input");

let mut name:u32 = match name.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
    };

let full_greeting = plus_one(&mut name);

println!("{}", full_greeting);
    
let abuse = plus_two(&mut name);

println!("{}", abuse);
}

fn plus_one(return1:&mut u32) -> &mut u32 {
    *return1 +=1;
    return1
}

fn plus_two(return2:&mut u32) -> &mut u32 {
    *return2 +=2; 
    return2
}
