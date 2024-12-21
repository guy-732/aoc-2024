use core::str;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Input<'a> {
    designs: Vec<&'a [u8]>,
    patterns: Vec<&'a [u8]>,
}

fn parse(input: &str) -> Input<'_> {
    let mut lines = input.lines();
    let patterns = lines.next().expect("First line did not exist");
    let patterns = patterns
        .split(',')
        .map(|pattern| pattern.trim().as_bytes())
        .collect_vec();

    Input {
        designs: lines
            .filter(|line| !line.is_empty())
            .map(str::as_bytes)
            .collect_vec(),
        patterns,
    }
}

fn part1_check_pattern_rec(
    design: &[u8],
    patterns: &[&[u8]],
    matched: usize,
    checked: &mut [bool],
) -> bool {
    if design.len() <= matched {
        return true;
    }

    if checked[matched] {
        return false;
    }

    let design_to_match = &design[matched..];
    for &pattern in patterns {
        if !design_to_match.starts_with(pattern) {
            continue;
        }

        if part1_check_pattern_rec(design, patterns, matched + pattern.len(), checked) {
            return true;
        }
    }

    checked[matched] = true;
    false
}

fn part1_check_pattern(design: &[u8], patterns: &[&[u8]]) -> bool {
    part1_check_pattern_rec(design, patterns, 0, &mut vec![false; design.len()])
}

fn part2_check_pattern_rec(
    design: &[u8],
    patterns: &[&[u8]],
    matched: usize,
    cache: &mut [u64],
) -> u64 {
    if design.len() <= matched {
        return 1;
    }

    if cache[matched] != u64::MAX {
        return cache[matched];
    }

    let design_to_match = &design[matched..];
    let mut current_sum = 0;
    for &pattern in patterns {
        if !design_to_match.starts_with(pattern) {
            continue;
        }

        current_sum += part2_check_pattern_rec(design, patterns, matched + pattern.len(), cache);
    }

    cache[matched] = current_sum;
    current_sum
}

fn part2_check_pattern(design: &[u8], patterns: &[&[u8]]) -> u64 {
    part2_check_pattern_rec(design, patterns, 0, &mut vec![u64::MAX; design.len()])
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let input = parse(input);
    input
        .designs
        .par_iter()
        // .inspect(|&&towel| println!("Doing {:?}", str::from_utf8(towel).expect("Not utf8???")))
        .filter(|&&towel| part1_check_pattern(towel, &input.patterns))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);
    input
        .designs
        .par_iter()
        .map(|&towel| part2_check_pattern(towel, &input.patterns))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 16);
    }
}
