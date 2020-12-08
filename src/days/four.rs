use regex::Regex;
use std::{collections::HashMap, collections::HashSet, fs};

const FILENAME: &str = "inputs/inputday4";

pub fn solve() {
    let input = fs::read_to_string(FILENAME).expect("Failed to read file");

    let re = Regex::new(r"([a-z]+:[^\s]+)").unwrap();

    let mut total: i32 = 0;
    let mut total_part2: i32 = 0;

    for password in input.split("\n\n") {
        match process_passport(&password, &re, false) {
            Ok(_) => total += 1,
            Err(_) => {}
        }
        match process_passport(&password, &re, true) {
            Ok(_) => total_part2 += 1,
            Err(_) => {}
        }
    }

    println!("Answer to day 4 part 1 is {}", total);
    println!("Answer to day 4 part 2 is {}", total_part2);
}

fn process_passport(pass: &str, re: &Regex, extra_checks: bool) -> Result<(), &'static str> {
    let matches = re.captures_iter(pass);

    let keys: HashMap<&str, &str> = matches
        .into_iter()
        .map(|captures| {
            let mut parts = captures
                .get(0)
                .expect("Failed to get capture")
                .as_str()
                .split(':');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .into_iter()
        .collect();

    let required_keys: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .into_iter()
        .collect();

    let missing_keys: HashSet<&str> = required_keys
        .iter()
        .filter(|&k| !keys.keys().any(|x| x == k))
        .map(|k| *k)
        .collect();

    if missing_keys.len() > 0 && !(missing_keys.len() == 1 && missing_keys.contains("cid")) {
        return Err("Missing fields");
    }

    if extra_checks {
        // byr
        let byr: i32 = keys["byr"].parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return Err("Byr incorrect");
        }

        // eyr
        let iyr: i32 = keys["iyr"].parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return Err("Iyr incorrect");
        }

        // eyr
        let eyr: i32 = keys["eyr"].parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return Err("Eyr incorrect");
        }

        // hgt
        let hgt = keys["hgt"];
        if hgt.ends_with("cm") {
            let n: i32 = hgt[0..hgt.len() - 2].parse().unwrap();
            if n < 150 || n > 193 {
                return Err("Height in cm incorrect");
            }
        } else if hgt.ends_with("in") {
            let n: i32 = hgt[0..hgt.len() - 2].parse().unwrap();
            if n < 59 || n > 76 {
                return Err("Height in in incorrect");
            }
        } else {
            return Err("Hgt wrong");
        }

        // hcl
        if !Regex::new(r"\#[a-f0-9]{6}").unwrap().is_match(keys["hcl"]) {
            return Err("Hcl no match");
        }

        // ecl
        let options: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .collect();
        if !options.contains(keys["ecl"]) {
            return Err("Ecl not found");
        }

        // pid
        if keys["pid"].len() != 9 {
            return Err("Incorrect length pid");
        }
        match keys["pid"].parse::<i32>() {
            Ok(_) => {}
            Err(_) => return Err("Failed to parse pid"),
        }
    }

    Ok(())
}
