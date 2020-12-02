use std::{io::BufReader, io::BufRead, fs::File, env};
use errors::{AppError, AppResult};

mod errors;

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

#[derive(Debug)]
struct PasswordPolicy {
    value: String,
    min_occurrences: usize,
    max_occurrences: usize,
}

#[derive(Debug)]
struct Password {
    value: String,
    policy: PasswordPolicy,
}

fn read_passwords(path: &str) -> AppResult<Vec<Password>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| {
            let parts = line?
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();

            let occurrences = parts
                .get(0)
                .expect("Line missing password policy")
                .split("-")
                .map(|x| x.parse::<usize>().expect("Failed to parse password policy"))
                .collect::<Vec<_>>();

            let min_occurrences = *occurrences.get(0).expect("Missing min occurrence in password policy");
            let max_occurrences = *occurrences.get(1).expect("Missing max occurrence in password policy");

            let policy_value = parts
                .get(1)
                .expect("Line missing password policy value")
                .replace(":", "");

            let value = parts.get(2).expect("Line missing password").to_owned();

            Ok(Password {
                value,
                policy: PasswordPolicy {
                    value: policy_value,
                    min_occurrences,
                    max_occurrences,
                },
            })
        })
        .collect()
}

fn run_day_one() -> AppResult<()> {
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

fn sled_rental_password_policy(passwords: &[Password]) -> AppResult<u32> {
    let invalid_password_count = passwords.iter().filter(|password| {
        let policy_value_count = password.value.split("").fold(0, |acc, x| match x == password.policy.value {
            true => acc + 1,
            false => acc,
        });

        policy_value_count >= password.policy.min_occurrences && policy_value_count <= password.policy.max_occurrences
    })
    .count();

    Ok(invalid_password_count as u32)
}



fn toboggan_corporate_password_policy(passwords: &[Password]) -> AppResult<u32> {
    let invalid_password_count = passwords.iter().filter(|password| {
        let Password { value, policy } = password;
        let first = value
            .get((policy.min_occurrences - 1)..policy.min_occurrences)
            .expect("Failed to parse password");

        let second = value
            .get((policy.max_occurrences - 1)..policy.max_occurrences)
            .expect("Failed to parse password");

        (first == policy.value) ^ (second == policy.value)
    })
    .count();

    Ok(invalid_password_count as u32)
}

fn run_day_two() -> AppResult<()> {
    let passwords = read_passwords("data/passwords.txt")?;

    println!("There are {} invalid sled rental password(s)", sled_rental_password_policy(&passwords)?);

    println!("There are {} invalid toboggan corporate password(s)", toboggan_corporate_password_policy(&passwords)?);

    Ok(())
}

fn main() -> AppResult<()> {
    let action = env::args().nth(1).expect("Must provide an argument");

    match action.trim() {
        "1" => run_day_one(),
        "2" => run_day_two(),
        _ => panic!(format!("Unknown day {}", action))
    }
}
