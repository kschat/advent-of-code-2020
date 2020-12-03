use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/toboggan-map.txt");

#[derive(Debug)]
enum Cell {
    Tree,
    Empty,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Board {
  cells: Vec<Vec<Cell>>,
  height: usize,
  width: usize,
}

impl Board {
  pub fn new(cells: Vec<Vec<Cell>>) -> Board{
    let height = cells.len();
    let width = cells.get(0).expect("Cells must contain at least 1 row").len();
    Board {
      cells,
      height,
      width,
    }
  }
}

#[derive(Debug)]
struct Slope {
  rise: usize,
  run: usize,
}

fn calculate_total_collisions(board: &Board, slope: &Slope) -> u64 {
  let mut position = Position { x: 0, y: 0 };

  let mut collision_count = 0;

  while position.y < board.height {
      position = Position {
          x: (position.x + slope.run) % board.width,
          y: position.y + slope.rise,
      };

      if position.y >= board.height {
          return collision_count;
      }

      let cell = board.cells
          .get(position.y)
          .expect("Out of bounds of map height")
          .get(position.x)
          .expect("Out of bounds of map width");

      collision_count += match cell {
          Cell::Tree => 1,
          _ => 0,
      };
  };

  collision_count
}

pub fn run() -> AppResult<()> {
    let cells = INPUT
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|character| match character {
                    '.' => Cell::Empty,
                    '#' => Cell::Tree,
                    value => panic!(format!("Unknown value \"{}\" in map", value)),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let board = Board::new(cells);

    let collision_count_1 = calculate_total_collisions(&board, &Slope {
      run: 1,
      rise: 1,
    });

    let collision_count_2 = calculate_total_collisions(&board, &Slope {
      run: 3,
      rise: 1,
    });

    let collision_count_3 = calculate_total_collisions(&board, &Slope {
      run: 5,
      rise: 1,
    });
    let collision_count_4 = calculate_total_collisions(&board, &Slope {
      run: 7,
      rise: 1,
    });

    let collision_count_5 = calculate_total_collisions(&board, &Slope {
      run: 1,
      rise: 2,
    });

    println!("Tree collisions for rise: 1, run: 1: {}", collision_count_1);
    println!("Tree collisions for rise: 1, run: 3: {}", collision_count_2);
    println!("Tree collisions for rise: 1, run: 5: {}", collision_count_3);
    println!("Tree collisions for rise: 1, run: 7: {}", collision_count_4);
    println!("Tree collisions for rise: 2, run: 1: {}", collision_count_5);
    println!("Answer: {}", collision_count_1 * collision_count_2 * collision_count_3 * collision_count_4 * collision_count_5);

    Ok(())
}
