use chrono::{NaiveDate};
use std::io;

fn main() {

    //get input for dates
    let mut date_input1 = String::new();
    io::stdin().read_line(&mut date_input1).expect("Error");
    let date_input1 = date_input1.trim();
    
    let mut date_input2 = String::new();    
    io::stdin().read_line(&mut date_input2).expect("Error");
    let date_input2 = date_input2.trim();

    // Define the two dates
    let date1 = NaiveDate::parse_from_str(date_input1, "%Y-%m-%d").expect("Failed to parse date"); 
    let date2 = NaiveDate::parse_from_str(date_input2, "%Y-%m-%d").expect("Failed to parse date");

    // Calculate the difference between the two dates
    let duration = date2.signed_duration_since(date1);
    
    // Get the number of days
    let days_between = duration.num_days();

    println!("Number of days between {} and {}: {}", date1, date2, days_between);
}
