use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/bus-notes.txt");

fn find_earliest_bus(departure_timestamp: u32, bus_schedule: &[Option<u32>]) -> (u32, u32) {
    bus_schedule
        .iter()
        .filter_map(|&id| id)
        .map(|id| (id, (departure_timestamp - (departure_timestamp % id)) + id))
        .fold((u32::MAX, u32::MAX), |min, (id, time)| match time < min.1 {
            true => (id, time),
            false => min,
        })
}

fn find_sequential_bus_schedule(bus_schedule: &[Option<u64>]) -> u64 {
    let mut bus_sequence = bus_schedule
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| bus.map(|x| (i as u64, x)))
        .collect::<Vec<_>>();

    bus_sequence.sort_by(|(_, a), (_, b)| b.cmp(a));

    let mut current_index = 1;
    let (offset, mut step) = bus_sequence
        .get(0)
        .expect("Bus schedule must have at least 1 entry");

    let mut time = step - offset;

    loop {
        let (i, x) = bus_sequence
            .get(current_index)
            .expect("Out of bounds when getting next bus");

        if (time + i) % x == 0 {
            current_index += 1;
            step *= x;
        }

        if current_index == bus_sequence.len() {
            return time;
        }

        time += step;
    }
}

pub fn run() -> AppResult<()> {
    let bus_notes = INPUT.split("\n").collect::<Vec<_>>();
    let departure_timestamp = bus_notes
        .get(0)
        .expect("Missing depature timestamp")
        .parse::<u32>()
        .expect("Unable to parse depature timestamp");

    let bus_schedule = bus_notes
        .get(1)
        .expect("Missing bus IDs")
        .split(",")
        .map(|id| match id {
            "x" => None,
            _ => Some(id.parse::<u32>().expect("Unable to parse ID as integer")),
        })
        .collect::<Vec<_>>();

    let (earliest_id, earliest_time) = find_earliest_bus(departure_timestamp, &bus_schedule);
    println!(
        "Part 1: \"{}\"",
        (earliest_time - departure_timestamp) * earliest_id
    );

    let sequential_schedule_start_time = find_sequential_bus_schedule(
        &bus_schedule
            .iter()
            .map(|x| x.map(|i| i as u64))
            .collect::<Vec<_>>(),
    );

    println!("Part 2: \"{}\"", sequential_schedule_start_time);

    Ok(())
}
