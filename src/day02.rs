use itertools::Itertools;

#[aoc_generator(day02)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i64>().expect("Could not parse int"))
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe_report(report: &[i64]) -> bool {
    let increasing = report[0] < report[1];

    for (&prev, &next) in report.iter().tuple_windows() {
        if (increasing && prev >= next) || (!increasing && prev <= next) {
            return false;
        }

        if prev.abs_diff(next) > 3 {
            return false;
        }
    }

    true
}

#[aoc(day02, part1)]
fn part1(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|&report| is_safe_report(report))
        .count()
}

fn is_safe_report_skip(report: &[i64], skip: usize) -> bool {
    let increasing = if skip == 0 {
        report[1] < report[2]
    } else if skip == 1 {
        report[0] < report[2]
    } else {
        report[0] < report[1]
    };

    for (&prev, &next) in report
        .iter()
        .enumerate()
        .filter_map(|(i, value)| if i == skip { None } else { Some(value) })
        .tuple_windows()
    {
        if (increasing && prev >= next) || (!increasing && prev <= next) {
            return false;
        }

        if prev.abs_diff(next) > 3 {
            return false;
        }
    }

    true
}

fn is_safe_report_with_dampener(report: &[i64]) -> bool {
    if is_safe_report(report) {
        return true;
    }

    (0..report.len()).any(|skip| is_safe_report_skip(report, skip))
}

#[aoc(day02, part2)]
fn part2(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|&report| is_safe_report_with_dampener(report))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4);
    }
}
