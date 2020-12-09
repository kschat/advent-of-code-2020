use std::collections::HashMap;

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/customs-answers.txt");

pub fn run() -> AppResult<()> {
    let answer1 = INPUT
        .split("\n\n")
        .map(|group| {
            let group = group.replace("\n", "");
            let mut answers = group.split_terminator("").skip(1).collect::<Vec<_>>();
            answers.sort();
            answers.dedup();
            answers.len()
        })
        .fold(0, |acc, x| acc + x);

    println!("Answer 1: \"{}\"", answer1);

    let answer2 = INPUT
        .split("\n\n")
        .map(|group| {
            let people_count = group.split("\n").count();
            let answer_totals = group.split("\n").fold(HashMap::new(), |mut acc, x| {
                for ch in x.chars() {
                    *acc.entry(ch).or_insert(0) += 1;
                }

                acc
            });

            answer_totals
                .iter()
                .filter(|(_, &count)| count == people_count)
                .count()
        })
        .fold(0, |acc, x| acc + x);

    println!("Answer 2: \"{}\"", answer2);

    Ok(())
}
