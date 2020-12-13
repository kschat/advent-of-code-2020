use std::iter::repeat_with;

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/boot-code.txt");

#[derive(Debug, Clone)]
enum IntCode {
    Accum(i32),
    Jump(i32),
    Noop(i32),
}

#[derive(Debug, Eq, PartialEq)]
enum RunState {
    Running,
    Cycle,
    Terminated,
}

#[derive(Debug)]
struct State {
    position: i32,
    accumulator: i32,
    run_state: RunState,
    int_codes: Vec<IntCode>,
}

impl State {
    pub fn new() -> Self {
        Self {
            position: 0,
            accumulator: 0,
            run_state: RunState::Running,
            int_codes: vec![],
        }
    }
}

fn run_program(boot_code: &Vec<(i32, IntCode)>) -> State {
    let mut boot_code = boot_code.clone();
    let mut state = State::new();

    while state.run_state == RunState::Running {
        if state.position as usize >= boot_code.len() {
            state.run_state = RunState::Terminated;
            return state;
        }

        let (call_count, int_code) = boot_code
            .get_mut(state.position as usize)
            .expect("Position out of bounds");

        if *call_count >= 1 {
            state.run_state = RunState::Cycle;
            return state;
        }

        match int_code {
            IntCode::Accum(value) => {
                state.accumulator += *value;
                state.position += 1;
                state.int_codes.push(IntCode::Accum(*value));
            }
            IntCode::Jump(value) => {
                state.position += *value;
                state.int_codes.push(IntCode::Jump(*value));
            }
            IntCode::Noop(value) => {
                state.int_codes.push(IntCode::Noop(*value));
                state.position += 1;
            }
        }

        *call_count += 1;
    }

    state
}

pub fn run() -> AppResult<()> {
    let boot_code = INPUT
        .split("\n")
        .map(|line| {
            let instruction = line.split(" ").collect::<Vec<_>>();
            let int_code = match *instruction.get(0).expect("Missing int code") {
                "nop" => IntCode::Noop(
                    instruction
                        .get(1)
                        .expect("Missing noop argument")
                        .parse::<i32>()
                        .expect("Failed to parse noop argument"),
                ),
                "acc" => IntCode::Accum(
                    instruction
                        .get(1)
                        .expect("Missing acc argument")
                        .parse::<i32>()
                        .expect("Failed to parse acc argument"),
                ),
                "jmp" => IntCode::Jump(
                    instruction
                        .get(1)
                        .expect("Missing jmp argument")
                        .parse::<i32>()
                        .expect("Failed to parse jmp argument"),
                ),
                value => panic!(format!("Unexpected int code \"{}\"", value)),
            };

            (0, int_code)
        })
        .collect::<Vec<_>>();

    let state = run_program(&boot_code);

    println!("Part 1 {:?}", state.accumulator);

    let mut count = 0;
    let result2 = repeat_with(|| boot_code.clone())
        .find_map(|mut int_codes| {
            int_codes[count] = match &int_codes[count] {
                (c, IntCode::Noop(value)) => (*c, IntCode::Jump(*value)),
                (c, IntCode::Jump(value)) => (*c, IntCode::Noop(*value)),
                (c, IntCode::Accum(value)) => (*c, IntCode::Accum(*value)),
            };

            count += 1;

            match run_program(&int_codes) {
                State {
                    run_state: RunState::Terminated,
                    accumulator,
                    ..
                } => Some(accumulator),
                _ => None,
            }
        })
        .expect("Failed to find valid program");

    println!("Part 2 {:?}", result2);

    Ok(())
}
