use std::u64;
use std::{fs, str::FromStr};

static FILENAME: &str = "inputs/inputday13";

#[derive(Debug, Copy, Clone)]
enum Bus {
    OOS,
    ID(u64),
}

#[derive(Debug)]
enum BusParseError {}

impl FromStr for Bus {
    type Err = BusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => return Ok(Bus::OOS),
            _ => return Ok(Bus::ID(s.parse().unwrap())),
        }
    }
}

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let (timestamp, buses) = parse_input(&input);

    println!("Answer to day 13 part 1 is {}", part1(timestamp, &buses))
}

fn parse_input(input: &str) -> (u64, Vec<Bus>) {
    let (timestamp_str, buses_str) = match input.lines().collect::<Vec<&str>>().as_slice() {
        &[timestamp, buses, ..] => (timestamp, buses),
        _ => unreachable!(),
    };

    (
        timestamp_str.parse().unwrap(),
        buses_str
            .split(',')
            .map(|x| x.parse::<Bus>().unwrap())
            .collect(),
    )
}

fn part1(arrival: u64, buses: &[Bus]) -> u64 {
    let mut min_wait = u64::MAX;
    let mut best_bus: Option<Bus> = None;
    for bus in buses {
        match &bus {
            Bus::OOS => {}
            Bus::ID(interval) => {
                let wait_time = interval - arrival % interval;
                if wait_time < min_wait {
                    min_wait = wait_time;
                    best_bus = Some(*bus);
                }
            }
        }
    }
    match best_bus {
        Some(Bus::ID(interval)) => interval * min_wait,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1};

    #[test]
    fn example_part1() {
        let input = "939
7,13,x,x,59,x,31,19";
        let (arrival, buses) = parse_input(&input);
        assert_eq!(part1(arrival, &buses), 295);
    }
}
