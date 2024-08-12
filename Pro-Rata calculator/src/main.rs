fn main() {
    use std::io;

    let mut monthly_cost = String::new();

    println!("Please enter the monthly cost:");
    io::stdin().read_line(&mut monthly_cost).expect("Error");
    let monthly_cost:f32 = monthly_cost.trim().parse().unwrap();

    let mut credit_time = String::new();

    println!("How many days are we crediting for?");
    io::stdin().read_line(&mut credit_time).expect("Error");
    let credit_time:f32 = credit_time.trim().parse().unwrap();

    let amount_used = (monthly_cost / 30.00) * credit_time;
    let comp_amount:f32 = monthly_cost - amount_used;
    println!("The prorata amount is {}", comp_amount);

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Error");
}
