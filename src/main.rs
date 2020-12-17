#[macro_use]
extern crate lazy_static;

use day_eight::run as run_day_eight;
use day_eleven::run as run_day_eleven;
use day_five::run as run_day_five;
use day_four::run as run_day_four;
use day_nine::run as run_day_nine;
use day_one::run as run_day_one;
use day_seven::run as run_day_seven;
use day_six::run as run_day_six;
use day_ten::run as run_day_ten;
use day_three::run as run_day_three;
use day_twelve::run as run_day_twelve;
use day_two::run as run_day_two;
use errors::AppResult;
use std::env;

mod day_eight;
mod day_eleven;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_twelve;
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
        "6" => run_day_six(),
        "7" => run_day_seven(),
        "8" => run_day_eight(),
        "9" => run_day_nine(),
        "10" => run_day_ten(),
        "11" => run_day_eleven(),
        "12" => run_day_twelve(),
        _ => panic!(format!("Unknown day {}", action)),
    }
}
