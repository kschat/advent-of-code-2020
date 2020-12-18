#[macro_use]
extern crate lazy_static;

use d01::run as run_d01;
use d02::run as run_d02;
use d03::run as run_d03;
use d04::run as run_d04;
use d05::run as run_d05;
use d06::run as run_d06;
use d07::run as run_d07;
use d08::run as run_d08;
use d09::run as run_d09;
use d10::run as run_d10;
use d11::run as run_d11;
use d12::run as run_d12;
use d13::run as run_d13;
use errors::AppResult;
use std::env;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod errors;

fn main() -> AppResult<()> {
    let action = env::args().nth(1).expect("Must provide an argument");

    match action.trim() {
        "1" => run_d01(),
        "2" => run_d02(),
        "3" => run_d03(),
        "4" => run_d04(),
        "5" => run_d05(),
        "6" => run_d06(),
        "7" => run_d07(),
        "8" => run_d08(),
        "9" => run_d09(),
        "10" => run_d10(),
        "11" => run_d11(),
        "12" => run_d12(),
        "13" => run_d13(),
        _ => panic!(format!("Unknown day {}", action)),
    }
}
