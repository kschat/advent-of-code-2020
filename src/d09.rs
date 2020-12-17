use std::{cmp, collections::HashSet};

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/xmas-data.txt");

fn has_match<'a>(value: &u64, preamble: &'a [u64]) -> Option<&'a u64> {
    let preamble_set = preamble.iter().collect::<HashSet<_>>();

    preamble_set
        .iter()
        .find_map(|&x| {
            let min = cmp::min(value, x);
            let max = cmp::max(value, x);
            preamble_set.get(&(max - min))
        })
        .map(|&x| x)
}

pub fn run() -> AppResult<()> {
    let encrypted_data = INPUT
        .split("\n")
        .map(|x| x.parse::<u64>().expect("Failed to parse line as u64"))
        .collect::<Vec<_>>();

    let window_size = 26;
    let mut location = window_size - 1;
    let mut sliding_window = encrypted_data.windows(window_size);

    let corrupt_location = sliding_window
        .find_map(|window| {
            match has_match(&window[window_size - 1], &window[..(window_size - 1)]) {
                Some(_) => {
                    location += 1;
                    None
                }
                None => Some(location),
            }
        })
        .expect("Could not find corrupt location");

    let corrupt_value = encrypted_data[corrupt_location];

    println!("Part 1: \"{:?}\"", corrupt_value);

    let mut cursor = corrupt_location;
    let mut size = 2;

    while cursor > 0 {
        let slice = &encrypted_data[(cursor - size)..cursor];
        size += 1;

        let sum = slice.iter().fold(0, |acc, x| acc + x);

        if sum == corrupt_value {
            let min = slice.iter().min().expect("No min value in range");
            let max = slice.iter().max().expect("No max value in range");
            println!("Part 2: \"{:?}\"", min + max);
            break;
        }

        if sum > corrupt_value {
            cursor -= 1;
            size = 2;
        }
    }

    Ok(())
}
