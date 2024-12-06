use itertools::Itertools;

fn count_xmas(stream: impl IntoIterator<Item = u8>) -> usize {
    const XMAS: &[u8] = b"XMAS";
    let mut current = 0;
    let mut count = 0;

    for c in stream {
        if XMAS[current] == c {
            current += 1;
            if current == XMAS.len() {
                current = 0;
                count += 1;
            }

            continue;
        }

        if XMAS[0] == c {
            current = 1;
        } else {
            current = 0;
        }
    }

    count
}

fn count_xmas_reversed(stream: impl IntoIterator<Item = u8>) -> usize {
    const XMAS: &[u8] = b"SAMX";
    let mut current = 0;
    let mut count = 0;

    for c in stream {
        if XMAS[current] == c {
            current += 1;
            if current == XMAS.len() {
                current = 0;
                count += 1;
            }

            continue;
        }

        if XMAS[0] == c {
            current = 1;
        } else {
            current = 0;
        }
    }

    count
}

fn count_horizontals(matrix: &[&[u8]]) -> usize {
    matrix
        .iter()
        .map(|&row| count_xmas(row.iter().copied()) + count_xmas_reversed(row.iter().copied()))
        .sum()
}

fn count_verticals(matrix: &[&[u8]]) -> usize {
    (0..matrix[0].len())
        .map(|col_idx| {
            count_xmas(matrix.iter().map(|&row| row[col_idx]))
                + count_xmas_reversed(matrix.iter().map(|&row| row[col_idx]))
        })
        .sum()
}

fn count_diagonals_down_right(matrix: &[&[u8]]) -> usize {
    let mut count = 0;
    for row_idx in 0..matrix.len() {
        let iter_length = (matrix.len() - row_idx).min(matrix[0].len());
        count += count_xmas((0..iter_length).map(|idx| matrix[row_idx + idx][idx]));
        count += count_xmas_reversed((0..iter_length).map(|idx| matrix[row_idx + idx][idx]));
    }

    for col_idx in 1..matrix[0].len() {
        let iter_length = (matrix[0].len() - col_idx).min(matrix.len());
        count += count_xmas((0..iter_length).map(|idx| matrix[idx][col_idx + idx]));
        count += count_xmas_reversed((0..iter_length).map(|idx| matrix[idx][col_idx + idx]));
    }

    count
}

fn count_diagonals_down_left(matrix: &[&[u8]]) -> usize {
    let mut count = 0;
    for row_idx in 0..matrix.len() {
        let iter_length = (matrix.len() - row_idx).min(matrix[0].len());
        count += count_xmas(
            (0..iter_length).map(|idx| matrix[row_idx + idx][matrix[0].len() - idx - 1]),
        );
        count += count_xmas_reversed(
            (0..iter_length).map(|idx| matrix[row_idx + idx][matrix[0].len() - idx - 1]),
        );
    }

    for col_idx in 1..matrix[0].len() {
        let iter_length = (matrix[0].len() - col_idx).min(matrix.len());
        count += count_xmas(
            (0..iter_length).map(|idx| matrix[idx][matrix[0].len() - (col_idx + idx) - 1]),
        );
        count += count_xmas_reversed(
            (0..iter_length).map(|idx| matrix[idx][matrix[0].len() - (col_idx + idx) - 1]),
        );
    }

    count
}

#[aoc(day04, part1)]
fn part1(input: &str) -> usize {
    let input = input.lines().map(str::as_bytes).collect_vec();

    count_horizontals(&input)
        + count_verticals(&input)
        + count_diagonals_down_right(&input)
        + count_diagonals_down_left(&input)
}

macro_rules! is_x_mas_helper {
    ($c: expr) => {
        matches!($c, b'M' | b'S')
    };
}

fn is_x_mas(matrix: &[&[u8]], center: (usize, usize)) -> bool {
    if matrix[center.0][center.1] != b'A' {
        return false;
    }

    if !is_x_mas_helper!(matrix[center.0 - 1][center.1 - 1]) {
        return false;
    }

    if !is_x_mas_helper!(matrix[center.0 + 1][center.1 + 1]) {
        return false;
    }

    if matrix[center.0 + 1][center.1 + 1] == matrix[center.0 - 1][center.1 - 1] {
        return false;
    }

    if !is_x_mas_helper!(matrix[center.0 + 1][center.1 - 1]) {
        return false;
    }

    if !is_x_mas_helper!(matrix[center.0 - 1][center.1 + 1]) {
        return false;
    }

    if matrix[center.0 + 1][center.1 - 1] == matrix[center.0 - 1][center.1 + 1] {
        return false;
    }

    true
}

#[aoc(day04, part2)]
fn part2(input: &str) -> usize {
    let input = input.lines().map(str::as_bytes).collect_vec();
    let col_length = input.len() - 1;
    let row_length = input[0].len() - 1;

    (1..col_length)
        .map(|row_idx| {
            (1..row_length)
                .filter(|&col_idx| is_x_mas(&input, (row_idx, col_idx)))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9);
    }
}
