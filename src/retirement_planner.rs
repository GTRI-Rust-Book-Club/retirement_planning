// Includes
use std::io;
use log::{info, debug};

pub fn interactive() {

    info!("Problem 4: Retirement Hopes");

    // Prompt for current age
    let mut initial_age = String::new();
    println!("Please input your current age: ");
    io::stdin()
        .read_line(&mut initial_age)
        .expect("Failure to read input for current age");
    let initial_age: u32 = initial_age.trim().parse().expect("Age must be a number");

    // Prompt for current savings
    let mut initial_savings = String::new();
    println!("Please input your current savings: ");
    io::stdin()
        .read_line(&mut initial_savings)
        .expect("Failure to read input for current savings");
    let initial_savings: f32 = initial_savings.trim().parse().expect("Savings must be a dollar amount");

    // Ideal savings by retirement
    let mut ideal_savings = String::new();
    println!("Please input your ideal savings by the time you retire: ");
    io::stdin()
        .read_line(&mut ideal_savings)
        .expect("Failure to read input for ideal savings");
    let ideal_savings: f32 = ideal_savings.trim().parse().expect("Ideal savings must be a dollar amount");

    // How much is the user inputting him/herself each year in addition to investment return
    let mut yearly_input = String::new();
    println!("Please input the amount you expect to contribute yourself towards retirement each year: ");
    io::stdin()
        .read_line(&mut yearly_input)
        .expect("Failure to read input for user contribution");
    let yearly_input: f32 = yearly_input.trim().parse().expect("User contribution must be a dollar amount");

    // Compute
    compute(initial_age, initial_savings, yearly_input, ideal_savings);
}

/// Compute your future retirement age
///
/// * `initial_age` - Initial age of user
/// * `initial_savings` - Initial savings of user in US dollars
/// * `annual_input` - How much the user plans to input (US dollars) on an annual basis
/// * `ideal_savings` - How much money the user wishes to retire with
/// * `avg_annual_return` - Average annual return TODO: Where did we get this number?
pub fn compute(initial_age: u32, initial_savings: f32, annual_input: f32,
               ideal_savings: f32) -> u32 {

    // Assume the average annual return is 5.6% with a mix of stocks, bonds, and cash.
    const AVG_ANNUAL_RETURN: f32 = 0.056;

    // Determine how much money we save each year
    // amount earned each year = current savings * (1.0 + AVG_ANNUAL_RETURN)
    let mut savings_amt: f32 = initial_savings;
    let mut retirement_age: u32 = initial_age;
    while savings_amt < ideal_savings {
        // NOTE: The return does not include the user's input for the CURRENT year
        savings_amt = annual_input + savings_amt * (1.0 + AVG_ANNUAL_RETURN);
        retirement_age += 1;
        debug!("${} at age {}", savings_amt, retirement_age);
    }

    // Print summary if user input and results
    info!("You are a {}-year-old with starting savings of ${} and saving ${} each year", initial_age, initial_savings, annual_input);
    info!("If you wish you save ${} by retirement, then you will retire at age {}", ideal_savings, retirement_age);
    return retirement_age;
}

/// Unit tests
#[cfg(test)]
mod tests {
    // Note this useful idion: importing names from outer (for mod tests) scope
    use super::*;  // THIS IS NECESSARY

    #[test]
    fn test1() {
        let age: u32 = 30;
        let initial_savings: f32 = 0.0;  // You have nothing to being with
        let yearly_input: f32 = 1000000.0;  // $1M (super saver!)
        let retirement_goal: f32 = 2000000.0;  // $2M
        let retirement_age = compute(age, initial_savings, yearly_input, retirement_goal);
        assert_eq!(retirement_age, 32);
    }
}