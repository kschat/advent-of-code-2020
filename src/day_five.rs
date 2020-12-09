use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/boarding-passes.txt");

#[derive(Debug, Eq)]
struct Seat {
    row: u8,
    column: u8,
    id: u32,
}

impl Seat {
    pub fn new(row: u8, column: u8) -> Seat {
        Seat {
            row,
            column,
            id: (row as u32 * 8) + column as u32,
        }
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn calculate_location(length: u8, partitions: &[&str]) -> u8 {
    let range = partitions
        .iter()
        .fold(0..(length - 1), |acc, &direction| match direction {
            "F" | "L" => acc.start..((acc.end + acc.start) / 2),
            "B" | "R" => (((acc.end + acc.start) / 2) + 1)..acc.end,
            _ => panic!(format!("Unknown direction \"{}\"", direction)),
        });

    if range.start != range.end {
        panic!("Invalid partition size");
    }

    range.start
}

pub fn run() -> AppResult<()> {
    let mut seats = INPUT
        .split("\n")
        .map(|line| {
            let row_partition = line[..7].split_terminator("").skip(1).collect::<Vec<_>>();
            let column_partition = line[7..].split_terminator("").skip(1).collect::<Vec<_>>();

            Seat::new(
                calculate_location(128, &row_partition),
                calculate_location(8, &column_partition),
            )
        })
        .collect::<Vec<_>>();

    seats.sort();

    println!("Largest seat ID: \"{}\"", seats.last().unwrap().id);

    let seat = seats
        .iter()
        .skip(1)
        .map(|x| x.id)
        .collect::<Vec<_>>()
        .chunks(3)
        .find_map(|chunk| {
            let first = chunk[0];
            let second = chunk[1];
            let third = chunk[2];

            match (first + 1 == second, third - 1 == second) {
                (false, true) => Some(first + 1),
                (true, false) => Some(third - 1),
                (true, true) | (false, false) => None,
            }
        })
        .unwrap();

    println!("Seat ID: \"{}\"", seat);

    Ok(())
}
