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
                registers.0 >>= combo_op(&registers, instructions[instr_ptr + 1]);
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
                registers.1 = registers.0 >> combo_op(&registers, instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
            Instruction::Cdv => {
                registers.2 = registers.0 >> combo_op(&registers, instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
        }
    }

    result
}

#[aoc(day17, part2)]
fn part2(input: &(Vec<Instruction>, (u64, u64, u64))) -> u64 {
    part2_logic(&input.0, (0, input.1 .1, input.1 .2), 0)
}

fn part2_logic(instructions: &[Instruction], registers: (u64, u64, u64), on_instr: usize) -> u64 {
    let mut min_found = u64::MAX;
    for i in 0..=7 {
        let reg_a = registers.0 | (i << (3 * on_instr));
        let registers = (reg_a, registers.1, registers.2);
        if on_instr >= 3 && on_instr < instructions.len() {
            if (reg_a >> (3 * 3)) != 0
                && !first_n_instructions_correct(instructions, registers, on_instr - 3)
            {
                continue;
            }
        } else if on_instr == instructions.len() {
            if !first_n_instructions_correct(instructions, registers, instructions.len()) {
                continue;
            }

            return reg_a;
        }

        let reg_a = part2_logic(instructions, registers, on_instr + 1);
        if reg_a < min_found {
            min_found = reg_a;
        }
    }

    min_found
}

fn first_n_instructions_correct(
    instructions: &[Instruction],
    mut registers: (u64, u64, u64),
    to_check: usize,
) -> bool {
    let mut current = 0;
    let mut instr_ptr = 0;

    while let Some(&instr) = instructions.get(instr_ptr) {
        match instr {
            Instruction::Adv => {
                registers.0 >>= combo_op(&registers, instructions[instr_ptr + 1]);
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
                // result.push((combo_op(&registers, instructions[instr_ptr + 1]) & 0b111) as u8);
                let num = (combo_op(&registers, instructions[instr_ptr + 1]) & 0b111) as u8;
                if num != instructions[current].into() {
                    return false;
                }

                current += 1;
                if current >= to_check {
                    return true;
                }

                instr_ptr += 2;
            }
            Instruction::Bdv => {
                registers.1 = registers.0 >> combo_op(&registers, instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
            Instruction::Cdv => {
                registers.2 = registers.0 >> combo_op(&registers, instructions[instr_ptr + 1]);
                instr_ptr += 2;
            }
        }
    }

    // not enough outputs...
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART1_EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    const PART2_EXAMPLE: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(PART2_EXAMPLE)), 117440);
    }
}
