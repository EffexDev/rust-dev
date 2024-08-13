use std::io;
use chrono::prelude::*;

fn main() {
    let mut tool_select = String::new();

    println!("Please select a tool:");
    println!("1 - Financial Hardship");
    println!("2 - Prorata Calculator");
    io::stdin().read_line(&mut tool_select).expect("Error)");
    let tool_select:u8 = tool_select.trim().parse().unwrap();

    if tool_select == 2 {

        //get input for dates
        println!("Enter the start of the billing period in format: YYYY-MM-DD");
        let mut date_input1 = String::new();
        io::stdin().read_line(&mut date_input1).expect("Error");
        let date_input1 = date_input1.trim();
    
        println!("Enter the service end date in format: YYYY-MM-DD");
        let mut date_input2 = String::new();    
        io::stdin().read_line(&mut date_input2).expect("Error");
        let date_input2 = date_input2.trim();

        // Define the two dates
        let date1 = NaiveDate::parse_from_str(date_input1, "%Y-%m-%d").expect("Failed to parse date"); 
        let date2 = NaiveDate::parse_from_str(date_input2, "%Y-%m-%d").expect("Failed to parse date");

        // Calculate the difference between the two dates
        let duration = date2.signed_duration_since(date1);
    
        // Get the number of days
        let days_between: f32 = duration.num_days() as f32;

        //accept a user input for monthly cost as a string and parse it into an f32
        let mut monthly_cost = String::new();
        println!("Please enter the monthly cost:");
        io::stdin().read_line(&mut monthly_cost).expect("Error");
        let monthly_cost:f32 = monthly_cost.trim().parse().unwrap();

        //math to work out the daily cost of the service and how much was owed by customer 
        let daily_cost = monthly_cost / 30.00;
        let actual_cost = daily_cost * days_between;
    
        //subtract the amount owed from the monthly amount to get the prorata amount
        let comp_amount:f32 = monthly_cost - actual_cost;
        println!("The prorata amount is {}", comp_amount);
    }
    else {
        println!("Failed");
    }

    //input to stop the cmd line exiting immediately
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Error");
}
