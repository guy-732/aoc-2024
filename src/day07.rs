use std::{error::Error, str::FromStr};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Mul,
    Concatenate,
}

impl Operator {
    fn execute_op_u64(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
            Self::Concatenate => format!("{lhs}{rhs}")
                .parse()
                .expect("Could not parse concatenation"),
        }
    }

    fn next_operator_part1(self) -> Result<Self, Self> {
        match self {
            Self::Add => Ok(Self::Mul),
            Self::Mul => Err(Self::Add),
            _ => panic!("{self:?} was neither Add not Mul"),
        }
    }

    fn next_operator_part2(self) -> Result<Self, Self> {
        match self {
            Self::Add => Ok(Self::Mul),
            Self::Mul => Ok(Self::Concatenate),
            Self::Concatenate => Err(Self::Add),
        }
    }
}

impl From<Operator> for char {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => '+',
            Operator::Mul => '*',
            Operator::Concatenate => '|',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Equation {
    result: u64,
    terms: Vec<u64>,
}

impl FromStr for Equation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, terms) = s.split_once(':').ok_or("Could nto split on ':'")?;

        Ok(Equation {
            result: result.parse()?,
            terms: terms
                .split_whitespace()
                .map(|term| term.parse::<u64>())
                .try_collect()?,
        })
    }
}

#[aoc_generator(day07)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| line.parse().expect("Could not parse equation"))
        .collect_vec()
}

fn check_with_operators(ops: &[Operator], equation: &Equation) -> bool {
    // if equation.result == 7290 {
    // println!("ops for 7290: {ops:?}");
    // }

    let mut result = equation.terms[0];
    for (i, &op) in ops.iter().enumerate() {
        result = op.execute_op_u64(result, equation.terms[i + 1]);
        if result > equation.result {
            return false;
        }
    }

    result == equation.result
}

fn part1_equation_valid_impl(
    operators: &mut [Operator],
    operator_offset: usize,
    equation: &Equation,
) -> bool {
    if operator_offset >= operators.len() {
        check_with_operators(&operators, equation)
    } else {
        loop {
            if part1_equation_valid_impl(operators, operator_offset + 1, equation) {
                return true;
            }

            match operators[operator_offset].next_operator_part1() {
                Ok(next_op) => operators[operator_offset] = next_op,
                Err(reset) => {
                    operators[operator_offset] = reset;
                    return false;
                }
            }
        }
    }
}

fn part1_equation_valid(equation: &Equation) -> bool {
    let mut operators = vec![Operator::Add; equation.terms.len() - 1];

    let res = part1_equation_valid_impl(&mut operators, 0, equation);
    // println!("{equation:?}: {res}");
    res
}

fn part2_equation_valid_impl(
    operators: &mut [Operator],
    operator_offset: usize,
    equation: &Equation,
) -> bool {
    if operator_offset >= operators.len() {
        check_with_operators(&operators, equation)
    } else {
        loop {
            if part2_equation_valid_impl(operators, operator_offset + 1, equation) {
                return true;
            }

            match operators[operator_offset].next_operator_part2() {
                Ok(next_op) => operators[operator_offset] = next_op,
                Err(reset) => {
                    operators[operator_offset] = reset;
                    return false;
                }
            }
        }
    }
}

fn part2_equation_valid(equation: &Equation) -> bool {
    let mut operators = vec![Operator::Add; equation.terms.len() - 1];

    let res = part2_equation_valid_impl(&mut operators, 0, equation);
    // println!("{equation:?}: {res}");
    res
}

#[aoc(day07, part1)]
fn part1(equations: &[Equation]) -> u64 {
    equations
        .par_iter()
        .filter(|&eq| part1_equation_valid(eq))
        .map(|eq| eq.result)
        .sum()
}

#[aoc(day07, part2)]
fn part2(equations: &[Equation]) -> u64 {
    equations
        .par_iter()
        .filter(|&eq| part2_equation_valid(eq))
        .map(|eq| eq.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
