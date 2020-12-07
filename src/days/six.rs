use std::{collections::HashSet, fs};

static FILENAME: &str = "inputs/inputday6";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let groups: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect();

    let mut total_part1 = 0;

    for group in &groups {
        total_part1 += group
            .iter()
            .map(|x| x.chars())
            .flatten()
            .collect::<HashSet<char>>()
            .len();
    }

    println!("Answer to day 6 part 1 is {}", total_part1);

    let mut total_part2 = 0;

    for group in &groups {
        let mut answers: Vec<HashSet<char>> = Vec::new();

        for line in group.iter().filter(|x| !x.is_empty()) {
            answers.push(line.chars().collect());
        }

        total_part2 += {
            let set = answers
                .iter()
                .fold(answers.get(0).unwrap().clone(), |acc, x| {
                    acc.intersection(&x).cloned().collect()
                });
            set.len()
        }
    }

    println!("Answer to day 6 part 2 is {}", total_part2);
}
