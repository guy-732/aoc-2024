use std::vec;

use fnv::FnvHashMap;
use itertools::zip_eq;

#[aoc(day01, part1)]
fn part1(input: &str) -> u64 {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    for line in input.lines() {
        let (l, r) = line.split_once(' ').expect("Could not split line");
        left.push(l.trim().parse().expect("Could not parse left num"));
        right.push(r.trim().parse().expect("Could not parse right num"));
    }

    left.sort();
    right.sort();
    zip_eq(left, right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn add_to_hash_map(num: u64, map: &mut FnvHashMap<u64, u64>) {
    if let Some(count) = map.get_mut(&num) {
        *count += 1;
    } else {
        map.insert(num, 1);
    }
}

#[aoc(day01, part2)]
fn part2(input: &str) -> u64 {
    let mut left = vec![];
    let mut right = FnvHashMap::default();

    for line in input.lines() {
        let (l, r) = line.split_once(' ').expect("Could not split line");
        left.push(l.trim().parse().expect("Could not parse left num"));
        add_to_hash_map(
            r.trim().parse().expect("Could not parse right num"),
            &mut right,
        );
    }

    left.into_iter()
        .map(|num| right.get(&num).unwrap_or(&0) * num)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}
