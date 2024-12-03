use regex::Regex;

#[aoc(day03, part1)]
fn part1(input: &str) -> u64 {
    let regex = Regex::new(r#"mul\((\d+),(\d+)\)"#).expect("Could not compile regex");
    regex
        .captures_iter(input)
        .map(|m| {
            let left: u64 = m
                .get(1)
                .expect("Capture group 1 missing")
                .as_str()
                .parse()
                .expect("Failed to parse int");
            let right: u64 = m
                .get(2)
                .expect("Capture group 2 missing")
                .as_str()
                .parse()
                .expect("Failed to parse int");
            left * right
        })
        .sum()
}

#[aoc(day03, part2)]
fn part2(input: &str) -> u64 {
    let regex =
        Regex::new(r#"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"#).expect("Could not compile regex");
    let mut enabled = true;
    regex
        .captures_iter(input)
        .map(|m| {
            let whole = m.get(0).unwrap().as_str();
            if whole == "don't()" {
                enabled = false;
                return 0;
            }

            if whole == "do()" {
                enabled = true;
                return 0;
            }

            if enabled {
                let left: u64 = m
                    .get(1)
                    .expect("Capture group 1 missing")
                    .as_str()
                    .parse()
                    .expect("Failed to parse int");
                let right: u64 = m
                    .get(2)
                    .expect("Capture group 2 missing")
                    .as_str()
                    .parse()
                    .expect("Failed to parse int");
                left * right
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const PART2_EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(PART1_EXAMPLE), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(PART2_EXAMPLE), 48);
    }
}
