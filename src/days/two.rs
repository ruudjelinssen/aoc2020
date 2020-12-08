use regex::Regex;
use std::fs;

const FILENAME: &str = "inputs/inputday2";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).expect("Failed to read file");

    let mut answer = 0;

    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z]+): (?P<password>[a-z]+)$")
        .expect("Regex failed to parse");

    for line in input.lines() {
        if line_valid_part1(&re, line) {
            answer += 1;
        }
    }

    println!("Answer to day 2 part 1 is {:?}", answer);

    answer = 0;
    for line in input.lines() {
        if line_valid_part2(&re, line) {
            answer += 1;
        }
    }

    println!("Answer to day 2 part 1 is {:?}", answer);
}

fn line_valid_part1(re: &Regex, line: &str) -> bool {
    let matches = re.captures(line).expect("Failed to parse line");

    let min = matches["min"].parse::<usize>().unwrap();
    let max = matches["max"].parse::<usize>().unwrap();
    let _char = matches["char"].parse::<char>().unwrap();
    let password: &str = &matches["password"];
    let count = password.matches(_char).count();

    count >= min && count <= max
}

fn line_valid_part2(re: &Regex, line: &str) -> bool {
    let matches = re.captures(line).expect("Failed to parse line");

    let pos1 = matches["min"].parse::<usize>().unwrap();
    let pos2 = matches["max"].parse::<usize>().unwrap();
    let _char = matches["char"].parse::<char>().unwrap();
    let password: &str = &matches["password"];

    let mut occurences: i32 = 0;
    for (pos, passchar) in password.chars().enumerate() {
        if (pos == pos1 - 1 || pos == pos2 - 1) && passchar == _char {
            occurences += 1;
        }
    }
    occurences == 1
}
