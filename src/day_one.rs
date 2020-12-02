use std::{io::BufReader, io::BufRead, fs::File};
use crate::errors::{AppResult, AppError};

fn read_expense_report(path: &str) -> AppResult<Vec<u32>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?
            .parse::<u32>()
            .map_err(AppError::from)
        )
        .collect()
}

fn find_matching_entries(report: &[u32], required_match_count: usize, matches: &[u32]) -> Option<u32> {
    if required_match_count <= 1 {
        let match_sum = matches.iter().fold(0, |acc, x| acc + x);
        return report.iter()
            .find(|&value| match_sum + value == 2020)
            .map(|found| matches.iter().fold(1, |acc, x| acc * x) * found);
    }

    report.iter().enumerate().find_map(|(current_index, &current_value)| {
        let updated_matches = &[matches, &[current_value]].concat();
        let new_report = &report.iter()
            .skip(current_index)
            .map(|&v| v)
            .collect::<Vec<_>>();

        find_matching_entries(
            new_report,
            required_match_count - 1,
            updated_matches,
        )
    })
}

fn find_two_matching_entries(report: &[u32]) -> Option<u32> {
    find_matching_entries(report, 2, &vec![])
}

fn find_three_matching_entries(report: &[u32]) -> Option<u32> {
    find_matching_entries(report, 3, &vec![])
}

pub fn run() -> AppResult<()> {
    let report = read_expense_report("data/expense-report.txt")?;

    println!("{}", match find_two_matching_entries(&report) {
        Some(code) => format!("The code for problem 1 is: \"{}\"", code),
        None => "Could not find a match".to_string(),
    });

    println!("{}", match find_three_matching_entries(&report) {
        Some(code) => format!("The code for problem 2 is: \"{}\"", code),
        None => "Could not find a match".to_string(),
    });

    Ok(())
}
