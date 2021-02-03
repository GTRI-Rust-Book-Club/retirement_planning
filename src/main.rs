// Modules
mod retirement_planner;
// Includes
use simple_logger::SimpleLogger;

fn main() {
    // Initialize logger
    // retirement_planner::python_test();
    SimpleLogger::new().init().unwrap();
    retirement_planner::interactive();
}
