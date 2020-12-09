#[macro_use]
extern crate lazy_static;

use day_five::run as run_day_five;
use day_four::run as run_day_four;
use day_one::run as run_day_one;
use day_three::run as run_day_three;
use day_two::run as run_day_two;
use errors::AppResult;
use std::env;

mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod errors;

fn main() -> AppResult<()> {
    let action = env::args().nth(1).expect("Must provide an argument");

    match action.trim() {
        "1" => run_day_one(),
        "2" => run_day_two(),
        "3" => run_day_three(),
        "4" => run_day_four(),
        "5" => run_day_five(),
        _ => panic!(format!("Unknown day {}", action)),
    }
}
