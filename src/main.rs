fn main() {
    use std::io;

    let mut amount_owing = String::new();

    println!("Please enter the current amount owing:");
    io::stdin().read_line(&mut amount_owing).expect("Error");
    let amount_owing:f32 = amount_owing.trim().parse().unwrap();
    println!("{}", amount_owing)
}
