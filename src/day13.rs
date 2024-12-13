use approx::abs_diff_eq;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ClawMachine {
    button_a: Position,
    button_b: Position,
    target: Position,
}

fn parse_button(button: char, s: &str) -> Position {
    let (x, y) = s
        .trim()
        .strip_prefix("Button ")
        .expect("Failed to strip \"Button \"")
        .strip_prefix(button)
        .expect("Failed to parse button A/B")
        .strip_prefix(": X+")
        .expect("Failed to strip \": X+\"")
        .split_once(", Y+")
        .expect("Failed to strip \", Y+\"");

    Position(
        x.parse().expect("Failed to parse int"),
        y.parse().expect("Failed to parse int"),
    )
}

fn parse_target(s: &str) -> Position {
    let (x, y) = s
        .trim()
        .strip_prefix("Prize: X=")
        .expect("Could not strip \"Prize: X=\"")
        .split_once(", Y=")
        .expect("Could not split on \", Y=\"");

    Position(
        x.parse().expect("Failed to parse int"),
        y.parse().expect("Failed to parse int"),
    )
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples()
        .map(|(button_a, button_b, target)| ClawMachine {
            button_a: parse_button('A', button_a),
            button_b: parse_button('B', button_b),
            target: parse_target(target),
        })
        .collect_vec()
}

fn get_integers_out(a: f64, b: f64) -> Option<(u64, u64)> {
    let a_closest = a.round();
    let b_closest = b.round();
    if a_closest < 0. || b_closest < 0. {
        return None;
    }

    if !abs_diff_eq!(a, a_closest, epsilon = 0.001) {
        return None;
    }

    if !abs_diff_eq!(b, b_closest, epsilon = 0.001) {
        return None;
    }

    Some((a_closest as u64, b_closest as u64))
}

const EXTRA_OFFSET: isize = 10_000_000_000_000;

impl ClawMachine {
    fn do_part1(&self) -> Option<u64> {
        let mat: Array2<f64> = array![
            [self.button_a.0 as f64, self.button_b.0 as f64],
            [self.button_a.1 as f64, self.button_b.1 as f64]
        ];

        let to_solve: Array1<f64> = array![self.target.0 as f64, self.target.1 as f64];

        let h = mat
            .solve_into(to_solve)
            .expect("ndarray_linalg solve error");

        let (a_presses, b_presses) = get_integers_out(h[0], h[1])?;

        Some(a_presses * 3 + b_presses)
    }

    fn do_part2(&self) -> Option<u64> {
        let mat: Array2<f64> = array![
            [self.button_a.0 as f64, self.button_b.0 as f64],
            [self.button_a.1 as f64, self.button_b.1 as f64]
        ];

        let to_solve: Array1<f64> = array![
            (self.target.0 + EXTRA_OFFSET) as f64,
            (self.target.1 + EXTRA_OFFSET) as f64
        ];

        let h = mat
            .solve_into(to_solve)
            .expect("ndarray_linalg solve error");

        let (a_presses, b_presses) = get_integers_out(h[0], h[1])?;

        Some(a_presses * 3 + b_presses)
    }
}

#[aoc(day13, part1)]
fn part1(machines: &[ClawMachine]) -> u64 {
    machines.iter().filter_map(ClawMachine::do_part1).sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[ClawMachine]) -> u64 {
    machines.iter().filter_map(ClawMachine::do_part2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }
}
