use std::fs;

static FILENAME: &str = "inputs/inputday10";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();
    let adapters: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    // Sort the input
    let mut sorted = vec![0; input.len()];
    sorted.clone_from_slice(&adapters);
    sorted.sort();

    // Push target on the list
    sorted.push(sorted.iter().max().unwrap() + 3);

    // Solve
    let output = part1(&sorted);
    println!("Answer to day 10 part 1 is {}", output);
}

fn part1(input: &[u64]) -> u64 {
    // Calculate the differences between each pair
    let mut differences = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    // Add the first value as well (costly operation I assume)
    differences.insert(0, *input.get(0).unwrap());

    differences.iter().filter(|&&x| x == 3).count() as u64
        * differences.iter().filter(|&&x| x == 1).count() as u64
}

fn part2(input: &[u64], current: usize) -> u64 {
    if current == input.len() - 1 {
        return 1;
    } else {
        input
            .iter()
            .enumerate()
            .skip(current)
            .take(3)
            .filter(|(_, &x)| x <= 3 + input[current])
            .fold(0, |acc, (i, &_)| acc + part2(input, i))
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_example() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part1(&input), 22 * 10)
    }

    #[test]
    fn part2_example() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part2(&input, 0), 19208)
    }
}
