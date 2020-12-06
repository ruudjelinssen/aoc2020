use itertools::Itertools;
use std::fs;

static FILENAME: &str = "inputs/inputday1";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();
    if let Some(answer) = input
        .lines()
        .filter_map(|c| c.parse::<i32>().ok())
        .combinations(2)
        .find(|comb| comb.iter().fold(0, |acc, x| acc + x) == 2020)
        .map(|comb| comb.iter().fold(1, |acc, x| acc * x))
    {
        println!("Answer for day 1 is: {}", answer);
    }
}
