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
        let mut answers: HashSet<char> = HashSet::new();

        for line in group {
            for q in line.chars() {
                answers.insert(q);
            }
        }
        total_part1 += answers.len();
    }

    println!("Answer to day 6 part 1 is {}", total_part1);

    let mut total_part2 = 0;

    for group in &groups {
        let mut answers: Vec<HashSet<char>> = Vec::new();

        for line in group {
            let mut set: HashSet<char> = HashSet::new();
            for q in line.chars() {
                set.insert(q);
            }

            if set.len() > 0 {
                answers.push(set);
            }
        }

        total_part2 += {
            let mut set = answers.get(0).cloned().unwrap();
            for s in answers {
                set = set.intersection(&s).cloned().collect();
            }
            set.len()
        }
    }

    println!("Answer to day 6 part 2 is {}", total_part2);
}
