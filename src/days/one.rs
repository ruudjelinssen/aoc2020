use itertools::Itertools;
use std::fs;

static FILENAME: &str = "inputs/inputday1";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    println!(
        "Answer to day 1 part 1 is {}",
        get_solution(&input, 2).unwrap()
    );

    println!(
        "Answer to day 1 part 2 is {}",
        get_solution(&input, 3).unwrap()
    );
}

fn get_solution(input: &str, combs: usize) -> Option<u32> {
    input
        .lines()
        .filter_map(|c| c.parse().ok())
        .combinations(combs)
        .find(|comb| comb.iter().fold(0, |acc, x| acc + x) == 2020)
        .map(|comb| comb.iter().fold(1, |acc, x| acc * x))
}
