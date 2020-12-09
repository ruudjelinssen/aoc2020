use itertools::Itertools;
use std::fs;

static FILENAME: &str = "inputs/inputday9";
const PART1_PREAMBLE: usize = 25;

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let answer = part1(&numbers, PART1_PREAMBLE);
    println!("Answer to day 9 part 1 is {}", answer.unwrap());

    let answer = part2(&numbers, PART1_PREAMBLE);
    println!("Answer to day 9 part 2 is {}", answer.unwrap());
}

fn part1(numbers: &[u64], preamble: usize) -> Option<u64> {
    for i in preamble..numbers.len() {
        let comb_found = numbers
            .iter()
            .skip(i - preamble)
            .take(preamble)
            .combinations(2)
            .find(|comb| comb.iter().fold(0, |acc, x| acc + **x) == numbers[i]);
        match comb_found {
            Some(_) => {}
            None => return Some(numbers[i]),
        }
    }
    None
}

fn part2(numbers: &[u64], preamble: usize) -> Option<u64> {
    if let Some(target) = part1(numbers, preamble) {
        for i in 0..numbers.len() {
            for n in 2..numbers.len() {
                let comb: Vec<u64> = numbers.iter().skip(i).take(n).map(|&x| x).collect();
                let sum: u64 = comb.iter().sum();
                match sum {
                    s if s == target => {
                        return Some(comb.iter().min().unwrap() + comb.iter().max().unwrap())
                    }
                    s if s > target => break,
                    _ => {}
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn example_part1() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(part1(&numbers, 5), Some(127));
    }

    #[test]
    fn example_part2() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(part2(&numbers, 5), Some(62));
    }
}
