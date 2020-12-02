use std::env;
use errors::AppResult;
use day_one::run as run_day_one;
use day_two::run as run_day_two;

mod errors;
mod day_one;
mod day_two;

fn main() -> AppResult<()> {
    let action = env::args().nth(1).expect("Must provide an argument");

    match action.trim() {
        "1" => run_day_one(),
        "2" => run_day_two(),
        _ => panic!(format!("Unknown day {}", action))
    }
}
