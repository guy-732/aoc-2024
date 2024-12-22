use std::{
    fs,
    io::{self, Write},
};

use itertools::Itertools;

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|num| num.parse().expect("Could not convert to number"))
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &[u64]) -> u64 {
    input.iter().map(|&num| part1_process_num(num)).sum()
}

const PRUNE_NUMBER: u64 = 16777216;
const PRUNE_MASK: u64 = PRUNE_NUMBER - 1;

const STEPS: u32 = 2000;

fn part1_process_num(mut num: u64) -> u64 {
    for _ in 0..STEPS {
        num ^= num * 64;
        num &= PRUNE_MASK;

        num ^= num / 32;

        num ^= num * 2048;
        num &= PRUNE_MASK;
    }

    num
}

type Sequence = [i8; 4];

#[aoc(day22, part2)]
fn part2(input: &[u64]) -> u64 {
    let price_deltas = input
        .iter()
        .map(|&num| part2_generate_prices(num))
        .collect_vec();

    // dump_seqs(&price_deltas);
    find_best_sequence(&price_deltas)
}

#[allow(unused)]
fn dump_seqs(seqs: &[Vec<(u8, i8)>]) {
    dump_seq(&seqs[1], 1).expect("Failed to write seq 1");
    dump_seq(&seqs[2], 2).expect("Failed to write seq 2");
    dump_seq(&seqs[3], 3).expect("Failed to write seq 3");
}

fn dump_seq(seq: &[(u8, i8)], seq_num: usize) -> io::Result<()> {
    let mut f = fs::File::create(format!("sequence-{seq_num}.txt"))?;
    write!(f, "{}", seq[0].0)?;
    for (num, _) in seq.iter().skip(1) {
        write!(f, ", {num}")?;
    }

    Ok(())
}

fn find_best_sequence(price_with_deltas: &[Vec<(u8, i8)>]) -> u64 {
    // Only 130k combinations... welp
    let mut sequence = [-9, -9, -9, -9];
    let mut max_profit = 0;
    while let Some(next_seq) = next_sequence(sequence) {
        // println!("{sequence:?} ==> {next_seq:?}");
        sequence = next_seq;

        let profit = price_with_deltas
            .iter()
            .map(|prices| first_matching_sequence(prices, sequence))
            .sum();

        if max_profit < profit {
            println!("Found profit {profit} with sequence {sequence:?}");
            // for deltas in price_with_deltas {
            // println!("    {}", first_matching_sequence(&deltas, sequence));
            // }
            // println!();

            max_profit = profit;
        }
    }

    max_profit
}

fn next_sequence(mut sequence: Sequence) -> Option<Sequence> {
    next_sequence_increment_pos(&mut sequence, 3);
    loop {
        if sequence[0] >= 9 && sequence[1] >= 1 {
            return None;
        }

        if !(-9..=9).contains(&(sequence[0] + sequence[1])) {
            next_sequence_increment_pos(&mut sequence, 1);
            sequence[2] = -9;
            sequence[3] = -9;
            continue;
        }

        if !(-9..=9).contains(&(sequence[0] + sequence[1] + sequence[2])) {
            next_sequence_increment_pos(&mut sequence, 2);
            sequence[3] = -9;
            continue;
        }

        if !(-9..=9).contains(&(sequence[0] + sequence[1] + sequence[2] + sequence[3])) {
            next_sequence_increment_pos(&mut sequence, 3);
            continue;
        }

        return Some(sequence);
    }
}

fn next_sequence_increment_pos(sequence: &mut Sequence, pos: usize) {
    if sequence[pos] < 9 {
        sequence[pos] += 1;
    } else if pos != 0 {
        sequence[pos] = -9;
        next_sequence_increment_pos(sequence, pos - 1);
    } else {
        *sequence = [9, 9, 9, 9];
    }
}

fn first_matching_sequence(prices: &[(u8, i8)], sequence: Sequence) -> u64 {
    for i in 3..prices.len() {
        if prices[i - 3].1 != sequence[0] {
            continue;
        }

        if prices[i - 2].1 != sequence[1] {
            continue;
        }

        if prices[i - 1].1 != sequence[2] {
            continue;
        }

        if prices[i].1 != sequence[3] {
            continue;
        }

        return prices[i].0 as u64;
    }

    0
}

fn part2_generate_prices(mut num: u64) -> Vec<(u8, i8)> {
    let mut result = Vec::with_capacity((STEPS - 1) as usize);
    let mut previous = (num % 10) as i8;
    for _ in 0..STEPS {
        num ^= num * 64;
        num &= PRUNE_MASK;

        num ^= num / 32;

        num ^= num * 2048;
        num &= PRUNE_MASK;

        let price = (num % 10) as u8;
        let price_i8 = price as i8;
        let diff = price_i8 - previous;
        previous = price_i8;

        result.push((price, diff));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
10
100
2024";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1, 2, 3, 2024]), 23);
    }
}
