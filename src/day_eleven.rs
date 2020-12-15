use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash, Hasher},
};

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/ferry-seats.txt");

// 1 2 3
// 4 X 6
// 7 8 9
const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn count_occupied(cells: &[Vec<char>], row_index: usize, cell_index: usize) -> usize {
    ADJACENT_OFFSETS
        .iter()
        .map(|&(r, c)| (row_index as isize + r, cell_index as isize + c))
        .map(|(r, c)| {
            let row: usize = r.try_into().ok()?;
            let col: usize = c.try_into().ok()?;
            cells.get(row)?.get(col)
        })
        .filter_map(|x| x)
        .filter(|&&x| x == '#')
        .count()
}

fn calculate_updates(cells: &[Vec<char>]) -> Vec<(usize, usize, char)> {
    cells
        .iter()
        .enumerate()
        .fold(vec![], |acc, (row_index, row)| {
            row.iter()
                .enumerate()
                .fold(acc, |mut acc, (cell_index, value)| match value {
                    'L' if count_occupied(&cells, row_index, cell_index) == 0 => {
                        acc.push((row_index, cell_index, '#'));
                        acc
                    }
                    '#' if count_occupied(&cells, row_index, cell_index) >= 4 => {
                        acc.push((row_index, cell_index, 'L'));
                        acc
                    }
                    _ => acc,
                })
        })
}

fn calculate_part1(cells: &mut [Vec<char>]) -> usize {
    let mut changed = true;
    while changed {
        let hash = compute_hash(&cells);

        for (row_index, cell_index, value) in calculate_updates(&cells) {
            cells[row_index][cell_index] = value;
        }

        changed = hash != compute_hash(&cells);
    }

    let occupied_seats = cells.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|cell| **cell == '#').count()
    });

    occupied_seats
}

pub fn run() -> AppResult<()> {
    let cells = INPUT
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let part1 = calculate_part1(&mut cells.clone());
    println!("Part1: \"{}\"", part1);

    Ok(())
}
