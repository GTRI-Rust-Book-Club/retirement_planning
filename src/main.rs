// Modules
mod retirement_planner;
// Includes
use simple_logger::SimpleLogger;

fn main() {
    // Initialize logger
    SimpleLogger::new().init().unwrap();
    retirement_planner::interactive();
}