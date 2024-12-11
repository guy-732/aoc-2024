use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Stone(u64);

impl std::fmt::Debug for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Stone {
    fn next_state(self) -> ModifyStoneResult {
        if self.0 == 0 {
            return ModifyStoneResult::ValueChanged(Self(1));
        }

        let digit_count = (self.0 as f64).log10().floor() as u32 + 1;
        if digit_count % 2 == 0 {
            let pow = 10_u64.pow(digit_count / 2);
            ModifyStoneResult::StoneSplit(Self(self.0 / pow), Self(self.0 % pow))
        } else {
            ModifyStoneResult::ValueChanged(Self(self.0 * 2024))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModifyStoneResult {
    ValueChanged(Stone),
    StoneSplit(Stone, Stone),
}

fn method1_do_cycle(stones: impl IntoIterator<Item = Stone>, result: &mut Vec<Stone>) {
    for stone in stones {
        match stone.next_state() {
            ModifyStoneResult::ValueChanged(new_stone) => result.push(new_stone),
            ModifyStoneResult::StoneSplit(stone1, stone2) => {
                result.extend_from_slice(&[stone1, stone2])
            }
        }
    }
}

#[allow(unused)]
fn method1_cycle_n_times(input: &str, iterations: usize) -> u64 {
    let mut stones: Vec<Stone> = input
        .split_whitespace()
        .map(|stone| Stone(stone.parse().expect("Could not parse u64")))
        .collect();

    // println!("Stones: {:?}", &stones);
    let mut result = vec![];
    for _ in 0..iterations {
        method1_do_cycle(stones, &mut result);
        stones = result;
        result = vec![];
        // println!("Stones: {:?}", &stones);
    }

    stones.len() as u64
}

const MAX_DEPTH_CACHED_THRESHOLD: usize = 5;

fn method2_recurse(
    stone: Stone,
    max_depth: usize,
    cache: &mut FnvHashMap<(Stone, usize), u64>,
) -> u64 {
    if let Some(answer) = cache.get(&(stone, max_depth)) {
        return *answer;
    }

    let result = stone.next_state();
    if max_depth < 2 {
        match result {
            ModifyStoneResult::ValueChanged(_) => 1,
            ModifyStoneResult::StoneSplit(_, _) => 2,
        }
    } else {
        match result {
            ModifyStoneResult::ValueChanged(new_stone) => {
                let result = method2_recurse(new_stone, max_depth - 1, cache);
                if max_depth > MAX_DEPTH_CACHED_THRESHOLD {
                    cache.insert((stone, max_depth), result);
                }

                result
            }
            ModifyStoneResult::StoneSplit(new_stone1, new_stone2) => {
                let result = method2_recurse(new_stone1, max_depth - 1, cache)
                    + method2_recurse(new_stone2, max_depth - 1, cache);
                if max_depth > MAX_DEPTH_CACHED_THRESHOLD {
                    cache.insert((stone, max_depth), result);
                }

                result
            }
        }
    }
}

fn method2_cycle_n_times(input: &str, iterations: usize) -> u64 {
    let stones = input
        .split_whitespace()
        .map(|stone| Stone(stone.parse().expect("Could not parse u64")))
        .collect_vec();

    if iterations == 0 {
        return stones.len() as u64;
    }

    let mut cache = FnvHashMap::default();

    stones
        .into_iter()
        .map(|stone| method2_recurse(stone, iterations, &mut cache))
        .sum()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    method2_cycle_n_times(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u64 {
    method2_cycle_n_times(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "0 1 10 99 999";
    const EXAMPLE2: &str = "125 17";

    #[test]
    fn method1_examples() {
        assert_eq!(method1_cycle_n_times(EXAMPLE1, 1), 7);
        assert_eq!(method1_cycle_n_times(EXAMPLE2, 6), 22);
        assert_eq!(method1_cycle_n_times(EXAMPLE2, 25), 55312);
    }

    #[test]
    fn method2_examples() {
        assert_eq!(method2_cycle_n_times(EXAMPLE1, 1), 7);
        assert_eq!(method2_cycle_n_times(EXAMPLE2, 6), 22);
        assert_eq!(method2_cycle_n_times(EXAMPLE2, 25), 55312);
    }
}
