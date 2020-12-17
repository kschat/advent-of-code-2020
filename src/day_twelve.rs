use std::convert::{TryFrom, TryInto};

use crate::errors::{AppError, AppResult};

const INPUT: &'static str = include_str!("../data/navigation-instructions.txt");

#[derive(Debug)]
enum NavInstruction {
    MoveNorth(i32),
    MoveEast(i32),
    MoveSouth(i32),
    MoveWest(i32),
    MoveForward(i32),
    TurnLeft(i32),
    TurnRight(i32),
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&Direction> for i32 {
    fn from(value: &Direction) -> Self {
        match *value {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        }
    }
}

impl TryFrom<i32> for Direction {
    type Error = AppError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::North),
            90 => Ok(Direction::East),
            180 => Ok(Direction::South),
            270 => Ok(Direction::West),
            value => Err(AppError::new(&format!(
                "Failed to convert \"{}\" to a Direction",
                value
            ))),
        }
    }
}

#[derive(Debug)]
struct DirectionalShip {
    direction: Direction,
    position: (i32, i32),
}

impl DirectionalShip {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            position: (0, 0),
        }
    }

    pub fn process_instruction(&mut self, instruction: &NavInstruction) {
        match *instruction {
            NavInstruction::MoveNorth(value) => self.position.0 += value,
            NavInstruction::MoveSouth(value) => self.position.0 -= value,
            NavInstruction::MoveEast(value) => self.position.1 += value,
            NavInstruction::MoveWest(value) => self.position.1 -= value,
            NavInstruction::MoveForward(value) => self.process_instruction(&match self.direction {
                Direction::North => NavInstruction::MoveNorth(value),
                Direction::South => NavInstruction::MoveSouth(value),
                Direction::East => NavInstruction::MoveEast(value),
                Direction::West => NavInstruction::MoveWest(value),
            }),
            NavInstruction::TurnRight(value) => {
                let direction = (i32::from(&self.direction) + value) % 360;
                self.direction = direction.try_into().unwrap();
            }
            NavInstruction::TurnLeft(value) => {
                let direction = ((360 + i32::from(&self.direction)) - value) % 360;
                self.direction = direction.try_into().unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct WaypointShip {
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl WaypointShip {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: (1, 10),
        }
    }

    pub fn process_instruction(&mut self, instruction: &NavInstruction) {
        match *instruction {
            NavInstruction::MoveNorth(value) => self.waypoint.0 += value,
            NavInstruction::MoveSouth(value) => self.waypoint.0 -= value,
            NavInstruction::MoveEast(value) => self.waypoint.1 += value,
            NavInstruction::MoveWest(value) => self.waypoint.1 -= value,
            NavInstruction::MoveForward(value) => {
                self.position.0 += value * self.waypoint.0;
                self.position.1 += value * self.waypoint.1;
            }
            NavInstruction::TurnRight(value) => {
                let mut degrees = value;
                while degrees > 0 {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                    degrees -= 90;
                }
            }
            NavInstruction::TurnLeft(value) => {
                let mut degrees = value;
                while degrees > 0 {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                    degrees -= 90;
                }
            }
        }
    }
}

fn part1(instructions: &[NavInstruction]) -> AppResult<()> {
    let ship = instructions.iter().fold(
        DirectionalShip::new(Direction::East),
        |mut ship, instruction| {
            (&mut ship).process_instruction(instruction);
            ship
        },
    );

    println!(
        "Part 1: \"{:?}\"",
        ship.position.0.abs() + ship.position.1.abs()
    );

    Ok(())
}

fn part2(instructions: &[NavInstruction]) -> AppResult<()> {
    let ship = instructions
        .iter()
        .fold(WaypointShip::new(), |mut ship, instruction| {
            (&mut ship).process_instruction(instruction);
            ship
        });

    println!(
        "Part 2: \"{:?}\"",
        ship.position.0.abs() + ship.position.1.abs()
    );

    Ok(())
}

pub fn run() -> AppResult<()> {
    let instructions = INPUT
        .split("\n")
        .map(|line| {
            let instruction = &line[0..1];
            let value = line[1..].parse::<i32>().expect("Missing instruction value");

            match instruction {
                "N" => NavInstruction::MoveNorth(value),
                "E" => NavInstruction::MoveEast(value),
                "S" => NavInstruction::MoveSouth(value),
                "W" => NavInstruction::MoveWest(value),
                "F" => NavInstruction::MoveForward(value),
                "L" => NavInstruction::TurnLeft(value),
                "R" => NavInstruction::TurnRight(value),
                value => panic!("Unexpected value \"{}\"", value),
            }
        })
        .collect::<Vec<_>>();

    part1(&instructions)?;

    part2(&instructions)?;

    Ok(())
}
