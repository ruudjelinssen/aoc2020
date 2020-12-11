use std::{collections::HashMap, fs};

static FILENAME: &str = "inputs/inputday10";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();
    let adapters: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    // Solve
    println!("Answer to day 10 part 1 is {}", part1(&adapters));
    println!("Answer to day 10 part 2 is {}", part2(&adapters));
}

fn part1(input: &[u64]) -> u64 {
    // Sort the input
    let mut sorted = vec![0; input.len()];
    sorted.clone_from_slice(&input);
    sorted.sort();

    // Push target on the list
    sorted.push(sorted.iter().max().unwrap() + 3);

    // Calculate the differences between each pair
    let mut differences = sorted.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    // Add the first value as well (costly operation I assume)
    differences.insert(0, *sorted.get(0).unwrap());

    differences.iter().filter(|&&x| x == 3).count() as u64
        * differences.iter().filter(|&&x| x == 1).count() as u64
}

fn part2(input: &[u64]) -> u64 {
    // Sort the input
    let mut sorted = vec![0; input.len()];
    sorted.clone_from_slice(&input);
    sorted.sort();

    // Push target on the list
    sorted.push(sorted.iter().max().unwrap() + 3);

    // Insert the initial jolts
    sorted.insert(0, 0);

    part2_count(&sorted, 0, &mut HashMap::new())
}

fn part2_count(sorted: &[u64], current: usize, mem_table: &mut HashMap<usize, u64>) -> u64 {
    if mem_table.contains_key(&current) {
        return *mem_table.get(&current).unwrap();
    }

    if current == sorted.len() - 1 {
        return 1;
    } else {
        let count = sorted
            .iter()
            .enumerate()
            .skip(current + 1)
            .take(3)
            .filter(|(_, &x)| x <= 3 + sorted[current])
            .fold(0, |acc, (i, _)| acc + part2_count(sorted, i, mem_table));

        mem_table.insert(current, count);

        return count;
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

        assert_eq!(part2(&input), 19208)
    }
}
