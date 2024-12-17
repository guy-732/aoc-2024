use std::error::Error;
use std::fmt::Write;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Instruction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err("Instruction::try_from(): value was > 7"),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u8 = s.parse()?;
        Ok(num.try_into()?)
    }
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => 4,
            Instruction::Out => 5,
            Instruction::Bdv => 6,
            Instruction::Cdv => 7,
        }
    }
}

impl From<Instruction> for u64 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => 4,
            Instruction::Out => 5,
            Instruction::Bdv => 6,
            Instruction::Cdv => 7,
        }
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> (Vec<Instruction>, (u64, u64, u64)) {
    let mut lines = input.lines();
    let regs = lines
        .by_ref()
        .take(3)
        .map(|line| {
            let (_, num) = line
                .split_once(':')
                .expect("Register line did not contain a ':'");
            num.trim()
                .parse::<u64>()
                .expect("Could not parse register value")
        })
        .collect_tuple()
        .expect("Could not collect into tuple of 3 elements");

    (parse_program(lines.nth(1).expect("No program line")), regs)
}

fn parse_program(program_line: &str) -> Vec<Instruction> {
    let program_digits = program_line
        .strip_prefix("Program: ")
        .expect("Did not find \"Program: \" at the start of the program line");
    program_digits
        .split(',')
        .map(|digit| digit.parse().expect("Could not parse instruction"))
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &(Vec<Instruction>, (u64, u64, u64))) -> String {
    let result = execute(&input.0, input.1);
    let mut out = format!("{}", result[0]);

    for c in result.into_iter().skip(1) {
        write!(out, ",{c}").expect("Failed to write a u8 to a String");
    }

    out
}

fn combo_op(registers: &(u64, u64, u64), combo: Instruction) -> u64 {
    let val: u8 = combo.into();
    match val {
        0..=3 => val as u64,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        7 => panic!("combo operand 7 is not valid"),
        _ => unreachable!("Instruction::into<u8>() returned a value > 7"),
    }
}

fn execute(instructions: &[Instruction], mut registers: (u64, u64, u64)) -> Vec<u8> {
    let mut result = vec![];
    let mut instr_ptr = 0;

    while let Some(&instr) = instructions.get(instr_ptr) {
        match instr {
            Instruction::Adv => {
                registers.0 /= 1 << combo_op(&registers, instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
            Instruction::Bxl => {
                registers.1 ^= u64::from(instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
            Instruction::Bst => {
                registers.1 = combo_op(&registers, instructions[instr_ptr + 1]) & 0b111;
                instr_ptr += 2;
            }
            Instruction::Jnz => {
                if registers.0 == 0 {
                    instr_ptr += 2;
                } else {
                    instr_ptr = u64::from(instructions[instr_ptr + 1]) as usize;
                }
            }
            Instruction::Bxc => {
                registers.1 ^= registers.2;
                instr_ptr += 2;
            }
            Instruction::Out => {
                result.push((combo_op(&registers, instructions[instr_ptr + 1]) & 0b111) as u8);
                instr_ptr += 2;
            }
            Instruction::Bdv => {
                registers.1 =
                    registers.0 / (1 << combo_op(&registers, instructions[instr_ptr + 1]));
                instr_ptr += 2;
            }
            Instruction::Cdv => {
                registers.2 =
                    registers.0 / (1 << combo_op(&registers, instructions[instr_ptr + 1]));
                instr_ptr += 2;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }
}
