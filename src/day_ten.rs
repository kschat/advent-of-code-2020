use cached::proc_macro::cached;

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/jolts-adapters.txt");

fn calculate_jolt_difference(adapters: &[u16]) -> u16 {
    let (_, one_jolt, three_jolt) =
        // skip 0
        adapters[1..]
            .iter()
            .fold((0, 0, 0), |acc, &current| match current - acc.0 {
                1 => (current, acc.1 + 1, acc.2),
                3 => (current, acc.1, acc.2 + 1),
                val => panic!(format!("Unexpected difference {}", val)),
            });

    one_jolt * three_jolt
}

#[cached]
fn calculate_permutation_count(adapters: Vec<u64>, outlet_jolts: u64) -> u64 {
    match adapters.len() {
        1 => match adapters[0] == outlet_jolts {
            true => return 1,
            false => return 0,
        },
        _ => adapters
            .iter()
            .enumerate()
            .skip(1)
            .take_while(|(_, &x)| x - adapters[0] <= 3)
            .fold(0, |acc, (i, _)| {
                acc + calculate_permutation_count(adapters[i..].to_vec(), outlet_jolts)
            }),
    }
}

pub fn run() -> AppResult<()> {
    let mut adapters = INPUT
        .split("\n")
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    adapters.push(0);
    adapters.sort();

    let outlet_jolts = adapters.last().unwrap() + 3;
    adapters.push(outlet_jolts);

    println!("Part 1: \"{}\"", calculate_jolt_difference(&adapters));

    let part2 = calculate_permutation_count(
        adapters.iter().map(|&x| x as u64).collect::<Vec<_>>(),
        outlet_jolts as u64,
    );

    println!("Part 2: \"{}\"", part2);

    Ok(())
}
