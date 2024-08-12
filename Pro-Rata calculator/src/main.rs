fn main() {
    use std::io;

    //accept a user input for monthly cost as a string and parse it into an f32
    let mut monthly_cost = String::new();
    println!("Please enter the monthly cost:");
    io::stdin().read_line(&mut monthly_cost).expect("Error");
    let monthly_cost:f32 = monthly_cost.trim().parse().unwrap();

    //accept input for for days used as a string and parse o an f32
    let mut credit_time = String::new();
    println!("How many days were used?");
    io::stdin().read_line(&mut credit_time).expect("Error");
    let credit_time:f32 = credit_time.trim().parse().unwrap();

    //math to work out the daily cost of the service and how much was owed by customer 
    let daily_cost = monthly_cost / 30.00;
    let actual_cost = daily_cost * credit_time;
    
    //subtract the amount owed from the monthly amount to get the prorata amount
    let comp_amount:f32 = monthly_cost - actual_cost;
    println!("The prorata amount is {}", comp_amount);

    //input to stop the cmd line exiting immediately
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Error");
}
