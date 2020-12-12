use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/boot-code.txt");

#[derive(Debug)]
enum IntCode {
    Accum(i32),
    Jump(i32),
    Noop,
}

#[derive(Debug)]
struct State {
    position: i32,
    accumulator: i32,
    cycle_detected: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            position: 0,
            accumulator: 0,
            cycle_detected: false,
        }
    }
}

pub fn run() -> AppResult<()> {
    let mut boot_code = INPUT
        .split("\n")
        .map(|line| {
            let instruction = line.split(" ").collect::<Vec<_>>();
            let int_code = match *instruction.get(0).expect("Missing int code") {
                "nop" => IntCode::Noop,
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

    let mut state = State::new();

    while !state.cycle_detected {
        let (call_count, int_code) = boot_code
            .get_mut(state.position as usize)
            .expect("Position out of bounds");

        if *call_count >= 1 {
            state.cycle_detected = true;
            break;
        }

        match int_code {
            IntCode::Accum(value) => {
                state.accumulator += *value;
                state.position += 1;
                println!(
                    "acc {: <6} | {} {}",
                    value, state.accumulator, state.position
                );
            }
            IntCode::Jump(value) => {
                state.position += *value;
                println!(
                    "jmp {: <6} | {} {}",
                    value, state.accumulator, state.position
                );
            }
            IntCode::Noop => {
                state.position += 1;
                println!("nop{: <7} | {} {}", "", state.accumulator, state.position);
            }
        }

        *call_count += 1;
    }

    println!("Acc \"{}\"", state.accumulator);

    Ok(())
}
