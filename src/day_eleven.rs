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
const ADJACENT_VELOCITY: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Rules {
    allow_seat_distance: bool,
    tolerated_occupied_seats: usize,
}

fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn get_cell_at<'a>(
    cells: &'a [Vec<char>],
    position: &(usize, usize),
    velocity: &(isize, isize),
) -> Option<((usize, usize), &'a char)> {
    let row: usize = (position.0 as isize + velocity.0).try_into().ok()?;
    let col: usize = (position.1 as isize + velocity.1).try_into().ok()?;
    let cell = cells.get(row)?.get(col)?;
    // how much data can I cram into a single return value?
    let result = ((row, col), cell);

    Some(result)
}

fn get_next_seat<'a>(
    cells: &'a [Vec<char>],
    position: &(usize, usize),
    velocity: &(isize, isize),
) -> Option<&'a char> {
    match get_cell_at(cells, position, velocity)? {
        (next, '.') => get_next_seat(cells, &next, velocity),
        (_, cell) => Some(cell),
    }
}

fn count_occupied(cells: &[Vec<char>], position: &(usize, usize), with_distance: bool) -> usize {
    ADJACENT_VELOCITY
        .iter()
        .map(|velocity| match with_distance {
            true => get_next_seat(cells, position, velocity),
            false => get_cell_at(cells, position, velocity).map(|(_, cell)| cell),
        })
        .filter_map(|x| x)
        .filter(|&&x| x == '#')
        .count()
}

fn calculate_updates(cells: &[Vec<char>], rules: &Rules) -> Vec<(usize, usize, char)> {
    let Rules {
        allow_seat_distance,
        tolerated_occupied_seats,
    } = rules;

    cells
        .iter()
        .enumerate()
        .fold(vec![], |acc, (row_index, row)| {
            row.iter()
                .enumerate()
                .fold(acc, |mut acc, (cell_index, value)| match value {
                    'L' if count_occupied(
                        cells,
                        &(row_index, cell_index),
                        *allow_seat_distance,
                    ) == 0 =>
                    {
                        acc.push((row_index, cell_index, '#'));
                        acc
                    }
                    '#' if count_occupied(
                        cells,
                        &(row_index, cell_index),
                        *allow_seat_distance,
                    ) >= *tolerated_occupied_seats =>
                    {
                        acc.push((row_index, cell_index, 'L'));
                        acc
                    }
                    _ => acc,
                })
        })
}

fn simulate_people_sitting_habits(cells: &mut [Vec<char>], rules: &Rules) -> usize {
    let mut changed = true;
    while changed {
        let hash = compute_hash(&cells);

        for (row_index, cell_index, value) in calculate_updates(&cells, rules) {
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
        .collect::<Vec<_>>();

    let part1 = simulate_people_sitting_habits(
        &mut cells.clone(),
        &Rules {
            allow_seat_distance: false,
            tolerated_occupied_seats: 4,
        },
    );

    println!("Part 1: \"{}\"", part1);

    let part2 = simulate_people_sitting_habits(
        &mut cells.clone(),
        &Rules {
            allow_seat_distance: true,
            tolerated_occupied_seats: 5,
        },
    );

    println!("Part 2: \"{}\"", part2);

    Ok(())
}
