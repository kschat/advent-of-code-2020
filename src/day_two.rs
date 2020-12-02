use std::{io::BufReader, io::BufRead, fs::File};
use crate::errors::AppResult;

#[derive(Debug)]
struct PasswordPolicy {
    value: char,
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
                .replace(":", "")
                .parse::<char>()
                .expect("Could not parse policy value as character");

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

fn sled_rental_password_policy(passwords: &[Password]) -> AppResult<u32> {
    let invalid_password_count = passwords.iter().filter(|password| {
        let policy_value_count = password.value.chars().fold(0, |acc, x| match x == password.policy.value {
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
            .chars()
            .nth(policy.min_occurrences - 1)
            .expect("Failed to parse password");

        let second = value
            .chars()
            .nth(policy.max_occurrences - 1)
            .expect("Failed to parse password");

        (first == policy.value) ^ (second == policy.value)
    })
    .count();

    Ok(invalid_password_count as u32)
}

pub fn run() -> AppResult<()> {
    let passwords = read_passwords("data/passwords.txt")?;

    println!("There are {} invalid sled rental password(s)", sled_rental_password_policy(&passwords)?);

    println!("There are {} invalid toboggan corporate password(s)", toboggan_corporate_password_policy(&passwords)?);

    Ok(())
}
