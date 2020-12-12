use std::{fs, str::FromStr};

use regex::Regex;

static FILENAME: &str = "inputs/inputday12";

#[derive(Debug)]
enum Action {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
enum ActionParseError {}

impl FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"(?P<action>[A-Z])(?P<units>[0-9]+)")
            .unwrap()
            .captures(s)
            .unwrap();

        Ok(match &captures["action"] {
            "N" => Action::N(captures["units"].parse().unwrap()),
            "S" => Action::S(captures["units"].parse().unwrap()),
            "E" => Action::E(captures["units"].parse().unwrap()),
            "W" => Action::W(captures["units"].parse().unwrap()),
            "F" => Action::F(captures["units"].parse().unwrap()),
            "R" => Action::R(captures["units"].parse().unwrap()),
            "L" => Action::L(captures["units"].parse().unwrap()),
            _ => panic!("Shouldn't be possible"),
        })
    }
}

#[derive(Debug)]
struct Ferry {
    direction: Direction,
    position: (i64, i64),
    waypoint: (i64, i64),
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            direction: Direction::E,
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn do_action(&mut self, action: Action) {
        match action {
            Action::N(units) => self.position = (self.position.0, self.position.1 + units),
            Action::E(units) => self.position = (self.position.0 + units, self.position.1),
            Action::S(units) => self.position = (self.position.0, self.position.1 - units),
            Action::W(units) => self.position = (self.position.0 - units, self.position.1),
            Action::L(units) => match self.direction {
                Direction::N => {
                    self.direction = vec![Direction::N, Direction::W, Direction::S, Direction::E]
                        [((units / 90) % 4) as usize];
                }
                Direction::E => {
                    self.direction = vec![Direction::E, Direction::N, Direction::W, Direction::S]
                        [((units / 90) % 4) as usize];
                }
                Direction::S => {
                    self.direction = vec![Direction::S, Direction::E, Direction::N, Direction::W]
                        [((units / 90) % 4) as usize];
                }
                Direction::W => {
                    self.direction = vec![Direction::W, Direction::S, Direction::E, Direction::N]
                        [((units / 90) % 4) as usize];
                }
            },
            Action::R(units) => match self.direction {
                Direction::N => {
                    self.direction = vec![Direction::N, Direction::E, Direction::S, Direction::W]
                        [((units / 90) % 4) as usize];
                }
                Direction::E => {
                    self.direction = vec![Direction::E, Direction::S, Direction::W, Direction::N]
                        [((units / 90) % 4) as usize];
                }
                Direction::S => {
                    self.direction = vec![Direction::S, Direction::W, Direction::N, Direction::E]
                        [((units / 90) % 4) as usize];
                }
                Direction::W => {
                    self.direction = vec![Direction::W, Direction::N, Direction::E, Direction::S]
                        [((units / 90) % 4) as usize];
                }
            },
            Action::F(units) => match self.direction {
                Direction::N => self.position = (self.position.0, self.position.1 + units),
                Direction::E => self.position = (self.position.0 + units, self.position.1),
                Direction::S => self.position = (self.position.0, self.position.1 - units),
                Direction::W => self.position = (self.position.0 - units, self.position.1),
            },
        }
    }

    fn do_waypoint(&mut self, action: Action) {
        match action {
            Action::N(units) => self.waypoint = (self.waypoint.0, self.waypoint.1 + units),
            Action::E(units) => self.waypoint = (self.waypoint.0 + units, self.waypoint.1),
            Action::S(units) => self.waypoint = (self.waypoint.0, self.waypoint.1 - units),
            Action::W(units) => self.waypoint = (self.waypoint.0 - units, self.waypoint.1),
            Action::L(units) => {
                for _ in 0..(units / 90) as usize {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                }
            }
            Action::R(units) => {
                for _ in 0..(units / 90) as usize {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                }
            }
            Action::F(units) => {
                self.position = (
                    self.position.0 + self.waypoint.0 * units,
                    self.position.1 + self.waypoint.1 * units,
                )
            }
        }
    }

    fn distance(&self) -> u64 {
        (self.position.0.abs() + self.position.1.abs()) as u64
    }
}

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    println!("Answer to day 12 part 1 is {}", part1(&input));
    println!("Answer to day 12 part 2 is {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let instructions: Vec<Action> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ferry = Ferry::new();
    for action in instructions {
        ferry.do_action(action);
    }
    ferry.distance()
}

fn part2(input: &str) -> u64 {
    let instructions: Vec<Action> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ferry = Ferry::new();
    for action in instructions {
        ferry.do_waypoint(action);
    }
    ferry.distance()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn example_part1() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn example_part2() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part2(&input), 286);
    }
}
