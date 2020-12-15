use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/ferry-seats.txt");

fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

// 1 2 3
// 4 5 6
// 7 8 9
fn count_adjacent_occupied_seats(
    cells: &[Vec<char>],
    row_index: usize,
    cell_index: usize,
) -> usize {
    let top_left = cells
        .get(row_index - 1)
        .and_then(|row| row.get(cell_index - 1));

    let top_middle = cells.get(row_index - 1).and_then(|row| row.get(cell_index));

    let top_right = cells
        .get(row_index - 1)
        .and_then(|row| row.get(cell_index + 1));

    let center_left = cells.get(row_index).and_then(|row| row.get(cell_index - 1));

    let center_right = cells.get(row_index).and_then(|row| row.get(cell_index + 1));

    let bottom_left = cells
        .get(row_index + 1)
        .and_then(|row| row.get(cell_index - 1));

    let bottom_middle = cells.get(row_index + 1).and_then(|row| row.get(cell_index));

    let bottom_right = cells
        .get(row_index + 1)
        .and_then(|row| row.get(cell_index + 1));

    [
        top_left,
        top_middle,
        top_right,
        center_left,
        center_right,
        bottom_left,
        bottom_middle,
        bottom_right,
    ]
    .iter()
    .filter_map(|x| *x)
    .filter(|&&x| x == '#')
    .count()
}

fn calculate_updates(cells: &[Vec<char>]) -> Vec<(usize, usize, char)> {
    cells
        .iter()
        .enumerate()
        .fold(vec![], |acc, (row_index, row)| {
            // skip padding
            if row_index == 0 || row_index == cells.len() - 1 {
                return acc;
            }

            row.iter()
                .enumerate()
                .fold(acc, |mut acc, (cell_index, value)| {
                    // skip padding and floor
                    if *value == '.' || *value == '0' {
                        return acc;
                    }

                    let adjacent_occupied_seats =
                        count_adjacent_occupied_seats(&cells, row_index, cell_index);

                    match value {
                        'L' if adjacent_occupied_seats == 0 => {
                            acc.push((row_index, cell_index, '#'));
                            acc
                        }
                        '#' if adjacent_occupied_seats >= 4 => {
                            acc.push((row_index, cell_index, 'L'));
                            acc
                        }
                        _ => acc,
                    }
                })
        })
}

pub fn run() -> AppResult<()> {
    let cells = INPUT
        .split("\n")
        .map(|line| [&['0'], &line.chars().collect::<Vec<_>>()[..], &['0']].concat())
        .collect::<Vec<Vec<_>>>();

    let padding = cells[0].iter().map(|_| '0').collect::<Vec<_>>();
    let mut cells: Vec<Vec<char>> = [&[padding.clone()], &cells[..], &[padding.clone()]].concat();

    let mut changed = true;
    while changed {
        let hash = compute_hash(&cells);

        let updates = calculate_updates(&cells);
        for (row_index, cell_index, value) in updates {
            cells[row_index][cell_index] = value;
        }

        changed = hash != compute_hash(&cells);
    }

    let occupied_seats = cells.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|cell| **cell == '#').count()
    });

    println!("Part1: \"{}\"", occupied_seats);

    Ok(())
}
