use std::{collections::HashSet, fs};

const FILENAME: &str = "inputs/inputday5";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).expect("Failed to read file");

    let mut seats: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut row = 0;
        let mut row_off = 64;
        let mut col = 0;
        let mut col_off = 4;
        for c in line.chars() {
            match c {
                'F' => row_off /= 2,
                'B' => {
                    row += row_off;
                    row_off /= 2
                }
                'L' => col_off /= 2,
                'R' => {
                    col += col_off;
                    col_off /= 2
                }
                _ => {}
            }
        }
        seats.push(row * 8 + col);
    }

    let max: i32 = *(seats.iter().max().unwrap());
    println!("Answer to day 5 part 1 is {}", max);

    let seats_part2: HashSet<i32> = seats.into_iter().collect();

    for i in 8..max {
        if seats_part2.contains(&(i - 1))
            && seats_part2.contains(&(i - 1))
            && !seats_part2.contains(&i)
        {
            println!("Answer to day 5 part 2 is {}", i);
        }
    }
}
