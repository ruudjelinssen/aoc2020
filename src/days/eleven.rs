use std::{fs, str::FromStr};

static FILENAME: &str = "inputs/inputday11";

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
    Unknown,
}

#[derive(Debug)]
struct Ferry {
    seats: Vec<Vec<Seat>>,
}

#[derive(Debug)]
enum FerryParseError {}

impl FromStr for Ferry {
    type Err = FerryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seats: Vec<Vec<Seat>> = Vec::new();
        for line in s.lines() {
            let row: Vec<Seat> = line
                .chars()
                .map(|x| match x {
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    '#' => Seat::Occupied,
                    _ => Seat::Unknown,
                })
                .collect();
            seats.push(row);
        }
        Ok(Ferry { seats })
    }
}

impl Ferry {
    /// Performs a single round for part 1. Returns None when no changes have
    /// been made. Returns Some(count as u64) stating the number of seats
    /// changed.
    fn round_part1(&mut self) -> Option<u64> {
        let mut changed = 0u64;
        let mut new_seats: Vec<Vec<Seat>> = Vec::new();

        // Iterate over rows
        for (i, row) in self.seats.iter().enumerate() {
            // Create new list of seats for each row
            let mut new_row = Vec::new();

            // Iterate over columns
            for (j, seat) in row.iter().enumerate() {
                new_row.push(match seat {
                    Seat::Empty => {
                        if self
                            .get_neighbours(i, j)
                            .iter()
                            .filter(|s| ***s == Seat::Occupied)
                            .count()
                            == 0
                        {
                            changed += 1;
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if self
                            .get_neighbours(i, j)
                            .iter()
                            .filter(|s| ***s == Seat::Occupied)
                            .count()
                            >= 4
                        {
                            changed += 1;
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    Seat::Floor => Seat::Floor,
                    Seat::Unknown => Seat::Unknown,
                })
            }

            // Add new seats to ferry
            new_seats.push(new_row);
        }
        self.seats = new_seats;
        if changed > 0 {
            return Some(changed);
        }
        None
    }

    /// Performs a single round for part 1. Returns None when no changes have
    /// been made. Returns Some(count as u64) stating the number of seats
    /// changed.
    fn round_part2(&mut self) -> Option<u64> {
        let mut changed = 0u64;
        let mut new_seats: Vec<Vec<Seat>> = Vec::new();

        // Iterate over rows
        for (i, row) in self.seats.iter().enumerate() {
            // Create new list of seats for each row
            let mut new_row = Vec::new();

            // Iterate over columns
            for (j, seat) in row.iter().enumerate() {
                new_row.push(match seat {
                    Seat::Empty => {
                        if self
                            .get_visible_seats(i, j)
                            .iter()
                            .filter(|s| **s == Seat::Occupied)
                            .count()
                            == 0
                        {
                            changed += 1;
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if self
                            .get_visible_seats(i, j)
                            .iter()
                            .filter(|s| **s == Seat::Occupied)
                            .count()
                            >= 5
                        {
                            changed += 1;
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    Seat::Floor => Seat::Floor,
                    Seat::Unknown => Seat::Unknown,
                })
            }

            // Add new seats to ferry
            new_seats.push(new_row);
        }
        self.seats = new_seats;
        if changed > 0 {
            return Some(changed);
        }
        None
    }
    fn get_neighbours(&self, row: usize, column: usize) -> Vec<&Seat> {
        let mut neighbours: Vec<&Seat> = Vec::new();
        for i in row as isize - 1..row as isize + 2 {
            if i >= 0 && i < self.seats.len() as isize {
                for j in column as isize - 1..column as isize + 2 {
                    if (i != row as isize || j != column as isize)
                        && j >= 0
                        && j < self.seats.get(i as usize).unwrap().len() as isize
                    {
                        neighbours
                            .push(self.seats.get(i as usize).unwrap().get(j as usize).unwrap())
                    }
                }
            }
        }
        neighbours
    }

    fn get_visible_seats(&self, row: usize, column: usize) -> Vec<Seat> {
        let mut visible_seats = Vec::new();
        for &dir in DIRECTIONS.iter() {
            visible_seats.push(self.get_first_seat_hit(row as isize, column as isize, dir));
        }
        visible_seats
    }

    fn get_first_seat_hit(&self, row: isize, column: isize, direction: (isize, isize)) -> Seat {
        let mut current = (row, column);
        loop {
            current = (current.0 + direction.0, current.1 + direction.1);
            if current.0 < 0 || current.0 >= self.seats.len() as isize {
                break;
            }

            let seat_row = self.seats.get(current.0 as usize).unwrap();
            if current.1 < 0 || current.1 >= seat_row.len() as isize {
                break;
            }

            match seat_row.get(current.1 as usize).unwrap() {
                Seat::Unknown => {}
                Seat::Occupied => return Seat::Occupied,
                Seat::Empty => return Seat::Empty,
                Seat::Floor => {}
            }
        }
        Seat::Unknown
    }

    fn count_occupied_seats(&self) -> u64 {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|seat| **seat == Seat::Occupied).count() as u64)
            .sum()
    }
}

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();
    println!("Answer to day 11 part 1 is {}", part1(&input));
    println!("Answer to day 11 part 2 is {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut ferry: Ferry = input.parse().unwrap();

    loop {
        match ferry.round_part1() {
            Some(_) => {}
            None => break,
        }
    }

    ferry.count_occupied_seats()
}

fn part2(input: &str) -> u64 {
    let mut ferry: Ferry = input.parse().unwrap();

    loop {
        match ferry.round_part2() {
            Some(_) => {}
            None => break,
        }
    }

    ferry.count_occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_example() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part1(input), 37);
    }

    #[test]
    fn part2_example() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part2(input), 26);
    }
}
