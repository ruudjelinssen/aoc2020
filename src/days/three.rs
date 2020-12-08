use std::fs;

const FILENAME: &str = "inputs/inputday3";
const TREE: char = '#';

pub fn solve() {
    let input = fs::read_to_string(FILENAME).expect("Failed to read file");

    let mut answer = calc_slope(&input, 3, 1);

    println!("Answer to day 3 part 1 is {}", answer);

    answer = 1;
    answer *= calc_slope(&input, 1, 1);
    answer *= calc_slope(&input, 3, 1);
    answer *= calc_slope(&input, 5, 1);
    answer *= calc_slope(&input, 7, 1);
    answer *= calc_slope(&input, 1, 2);

    println!("Answer to day 3 part 2 is {}", answer);
}

pub fn calc_slope(input: &String, right: usize, down: usize) -> usize {
    let rows: Vec<&str> = input.lines().collect();

    let mut col: usize = 0;
    let mut answer: usize = 0;

    for i in (0..rows.len()).step_by(down) {
        let row = rows.get(i).expect("Failed to get row");
        if row.chars().nth(col).expect("Couldn't fetch the value") == TREE {
            answer += 1;
        }

        col = (col + right) % row.chars().count();
    }

    answer
}
