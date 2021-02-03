// Includes
use std::io;
use log::{info, debug};

use pyo3::prelude::*;
//use pyo3::types::IntoPyDict; // Used by demo code from PyO3
use pyo3::types::PyList;
use pyo3::PyNativeType;

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
    compute(initial_age, initial_savings, yearly_input, ideal_savings, true);
}

/// Compute your future retirement age
///
/// * `initial_age` - Initial age of user
/// * `initial_savings` - Initial savings of user in US dollars
/// * `annual_input` - How much the user plans to input (US dollars) on an annual basis
/// * `ideal_savings` - How much money the user wishes to retire with
/// * `avg_annual_return` - Average annual return TODO: Where did we get this number?
pub fn compute(initial_age: u32, initial_savings: f32, annual_input: f32,
               ideal_savings: f32, plot_data: bool) -> u32 {
    // Assume the average annual return is 5.6% with a mix of stocks, bonds, and cash.
    const AVG_ANNUAL_RETURN: f32 = 0.056;

    // Determine how much money we save each year
    // amount earned each year = current savings * (1.0 + AVG_ANNUAL_RETURN)
    let mut savings_amt: f32 = initial_savings;
    let mut retirement_age: u32 = initial_age;
    let mut record: Vec<f32> = vec![savings_amt];
    while savings_amt < ideal_savings {
        // NOTE: The return does not include the user's input for the CURRENT year
        savings_amt = annual_input + savings_amt * (1.0 + AVG_ANNUAL_RETURN);
        retirement_age += 1;
        debug!("${:.2} at age {}", savings_amt, retirement_age);
        record.push(savings_amt);
    }

    if plot_data {  // Plot
        // Use borrow passing so that the record can be sent to two functions
        let _res = plot(initial_age, &record);
        match python_plot(initial_age, &record) {
            Ok(_result) => debug!("Successful execution of python_plot"),
            Err(e) => debug!("There was an error {}", e),
        }
    }

    // Print summary if user input and results
    info!("You are a {}-year-old with starting savings of ${} and saving ${} each year", initial_age, initial_savings, annual_input);
    info!("If you wish you save ${} by retirement, then you will retire at age {}", ideal_savings, retirement_age);
    return retirement_age;
}

use plotters::prelude::*;

/// Plot savings towards retirement
///
/// * `initial_age` - Initial age of user
/// * `savings_history` - Projected savings history for user
pub fn plot(initial_age: u32, savings_history: &Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let filename: &str = "retirement_savings.png";
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let age_range: Vec<u32> = (initial_age..initial_age + savings_history.len() as u32).collect();

    // Define chart builder axes
    let mut chart = ChartBuilder::on(&root)
        .caption("Saving for Retirement", ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(80)
        .margin(10)
        .build_cartesian_2d(age_range[0]..*age_range.last().unwrap(),
                            savings_history[0]..*savings_history.last().unwrap())?;

    // Area chart
    chart.draw_series(
        AreaSeries::new(
            age_range.iter().zip(savings_history.iter()).map(|(x, y)| (*x, *y)),
            0.0,
            &BLUE.mix(0.2),
        ).border_style(&BLUE),
    )?;

    // Markers
    chart.draw_series(
        age_range.iter().zip(savings_history.iter()).map(|(x, y)| Circle::new((*x, *y), 5, BLUE.filled())),
    )?;

    // Draw labels and mesh
    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("Savings (US Dollars)")
        .x_desc("Age")
        .axis_desc_style(("sans-serif", 18))
        .draw()?;

    info!("Plotted savings history to {}", filename);
    Ok(())
}

/*
// This is an example case from the PyO3 documentation
pub fn python_test() -> PyResult<()> {
   let gil = Python::acquire_gil();
   let py = gil.python();
   let sys = py.import("sys")?;
   let version: String = sys.get("version")?.extract()?;

   let locals = [("os", py.import("os")?)].into_py_dict(py);
   let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
   let user: String = py.eval(code, None, Some(&locals))?.extract()?;

    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}
 */

pub fn python_plot(initial_age: u32, savings_history: &Vec<f32>) -> PyResult<()> {
    debug!("Attempting to run the python_plot code");
    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys")?;
        unsafe{
            match PyList::unchecked_downcast(sys.get("path")?).insert(0, "../../retirement_planning/src") {
                Ok(_result) => debug!("Successfully updated Python path"),
                Err(e) => debug!("Error updating python path {}", e),
            };
        }
        let path: Vec<String> = sys.get("path")?.extract()?;
        println!("The current path is");
        for folder in path {
            println!("{}", folder);
        }
        let retire_plot = PyModule::import(py, "retire_plot")?;
        retire_plot.call1("test", ())?;
        let plot_data = savings_history.clone();   // Since reference is borrowed, make a copy
        retire_plot.call1("plot", (initial_age, plot_data))?;
        Ok(())
    })
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
        let retirement_age = compute(age, initial_savings, yearly_input, retirement_goal, false);
        assert_eq!(retirement_age, 32);
    }
}
