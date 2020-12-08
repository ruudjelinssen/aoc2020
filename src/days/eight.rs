use itertools::Itertools;
use std::{collections::HashSet, fs};

const FILENAME: &str = "inputs/inputday8";

#[derive(PartialEq)]
enum InstructionType {
    ACC,
    JMP,
    NOP,
}

struct Instruction {
    instruction: InstructionType,
    operand: i32,
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        let (instruction_type, operand): (&str, &str) = line.split(' ').next_tuple().unwrap();
        Self {
            instruction: match instruction_type {
                "jmp" => InstructionType::JMP,
                "acc" => InstructionType::ACC,
                "nop" => InstructionType::NOP,
                _ => panic!("Failed to parse instruction type"),
            },
            operand: operand.parse().unwrap(),
        }
    }
}

pub fn solve() {
    let input = fs::read_to_string(FILENAME).expect("Failed to read file");

    let instructions = input
        .lines()
        .map(|line| Instruction::from_line(&line))
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut program_counter: i32 = 0;
    let mut instructions_done = HashSet::new();
    let mut accumulator: i32 = 0;

    loop {
        match instructions.get(program_counter as usize).unwrap() {
            Instruction {
                instruction,
                operand,
            } => match instruction {
                InstructionType::ACC => {
                    accumulator += operand;
                    program_counter += 1
                }
                InstructionType::JMP => program_counter += operand,
                InstructionType::NOP => program_counter += 1,
            },
        }
        if !instructions_done.insert(program_counter) {
            break;
        }
    }

    println!("Answer to day 8 part 1 is {}", accumulator);
}

fn part2(instructions: &Vec<Instruction>) {
    // Loop over all possible indices that can be changed
    for i in instructions.iter().positions(|x| {
        (x.instruction == InstructionType::JMP) || x.instruction == InstructionType::NOP
    }) {
        let mut program_counter: i32 = 0;
        let mut instructions_done = HashSet::new();
        let mut accumulator: i32 = 0;

        loop {
            let is_last_instruction = program_counter as usize == (instructions.len() - 1);

            match instructions.get(program_counter as usize).unwrap() {
                Instruction {
                    instruction,
                    operand,
                } => match instruction {
                    InstructionType::ACC => {
                        accumulator += operand;
                        program_counter += 1
                    }
                    InstructionType::JMP => {
                        if program_counter as usize == i {
                            program_counter += 1;
                        } else {
                            program_counter += operand;
                        }
                    }
                    InstructionType::NOP => {
                        if program_counter as usize == i {
                            program_counter += operand;
                        } else {
                            program_counter += 1;
                        }
                    }
                },
            };

            if !instructions_done.insert(program_counter) {
                break;
            }

            if is_last_instruction {
                println!("Answer to day 8 part 2 is {}", accumulator);
                return;
            }
        }
    }
}
