use std::{collections::HashSet, fs};

use regex::{Captures, Regex};

static FILENAME: &str = "inputs/inputday7";

#[derive(Debug)]
struct Rule {
    bag_type: String,
    contains: Vec<String>,
    contains_set: HashSet<String>,
}

pub fn solve() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let mut rules = Vec::new();

    for line in input.lines() {
        let rule = process_line(line);
        rules.push(rule);
    }

    let mut valid_bags: HashSet<String> = vec![String::from("shiny gold")].into_iter().collect();

    loop {
        let count = valid_bags.len();
        for rule in &rules {
            if rule.contains_set.intersection(&valid_bags).count() > 0 {
                valid_bags.insert(rule.bag_type.to_string());
            }
        }
        if count == valid_bags.len() {
            break;
        }
    }

    println!("Answer to day 7 part 1 is {}", valid_bags.len() - 1);

    let mut total: i32 = 0;

    bags_needed(&rules, &String::from("shiny gold"), &mut total);

    println!("Answer to day 7 part 2 is {}", total);
}

fn process_line(line: &str) -> Rule {
    let captures = Regex::new(r"(?P<bag_type>[a-z]+ [a-z]+) bags contain (?P<contains>.*)")
        .unwrap()
        .captures(line)
        .unwrap();

    let contains_captures: Vec<Captures> =
        Regex::new(r"(?P<number>\d+) (?P<type>[a-z]+ [a-z]+) bags?")
            .unwrap()
            .captures_iter(&captures["contains"])
            .collect();

    let mut contains = Vec::new();
    for cap in contains_captures {
        for _ in 0..cap["number"].parse::<i32>().unwrap() {
            contains.push(cap["type"].to_string());
        }
    }

    Rule {
        bag_type: captures["bag_type"].to_string(),
        contains: contains.clone(),
        contains_set: contains.into_iter().collect(),
    }
}

fn bags_needed(rules: &Vec<Rule>, bag_type: &String, total: &mut i32) {
    for rule in rules {
        if rule.bag_type.eq(bag_type) {
            for bag_contained in &rule.contains {
                *total += 1;
                bags_needed(rules, &bag_contained, total);
            }
        }
    }
}
