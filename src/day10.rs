use fnv::FnvHashSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

const DIRECT_NEIGHBORS: [Position; 4] = [
    Position(1, 0),
    Position(0, 1),
    Position(-1, 0),
    Position(0, -1),
];

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Height {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Invalid,
}

impl Height {
    fn is_uphill_by_one(self, other: Self) -> bool {
        match self {
            Self::Zero => matches!(other, Self::One),
            Self::One => matches!(other, Self::Two),
            Self::Two => matches!(other, Self::Three),
            Self::Three => matches!(other, Self::Four),
            Self::Four => matches!(other, Self::Five),
            Self::Five => matches!(other, Self::Six),
            Self::Six => matches!(other, Self::Seven),
            Self::Seven => matches!(other, Self::Eight),
            Self::Eight => matches!(other, Self::Nine),
            _ => false,
        }
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zero => '0',
                Self::One => '1',
                Self::Two => '2',
                Self::Three => '3',
                Self::Four => '4',
                Self::Five => '5',
                Self::Six => '6',
                Self::Seven => '7',
                Self::Eight => '8',
                Self::Nine => '9',
                Self::Invalid => '.',
            }
        )
    }
}

impl From<u8> for Height {
    fn from(value: u8) -> Self {
        if !value.is_ascii_digit() {
            return Self::Invalid;
        }

        match value - b'0' {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => unreachable!(
                "value - b'0' was not in 0..=9 despite value ({}) being in b'0'..=b'9'",
                value as char
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeightMap {
    map: Vec<Vec<Height>>,
}

impl HeightMap {
    fn get(&self, index: Position) -> Option<&Height> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.map.get(index.0 as usize)?.get(index.1 as usize)
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> {
        let row_len = self.map[0].len();
        (0..self.map.len()).flat_map(move |row_idx| {
            (0..row_len).map(move |col_idx| Position(row_idx as isize, col_idx as isize))
        })
    }

    fn count_part1_paths(&self) -> usize {
        let mut head_positions = FnvHashSet::default();
        self.iter_positions()
            .filter_map(|pos| {
                if self[pos] == Height::Zero {
                    self.fill_trailhead_positions(pos, &mut head_positions);
                    let result = head_positions.len();
                    head_positions.clear();
                    Some(result)
                } else {
                    None
                }
            })
            .sum()
    }

    fn fill_trailhead_positions(
        &self,
        current: Position,
        head_positions: &mut FnvHashSet<Position>,
    ) {
        if self[current] == Height::Nine {
            head_positions.insert(current);
            return;
        }

        for neighbor in DIRECT_NEIGHBORS {
            if let Some(&other) = self.get(current + neighbor) {
                if self[current].is_uphill_by_one(other) {
                    self.fill_trailhead_positions(current + neighbor, head_positions);
                }
            }
        }
    }

    fn count_part2_paths(&self) -> usize {
        self.iter_positions()
            .filter_map(|pos| {
                if self[pos] == Height::Zero {
                    Some(self.count_part2_paths_impl(pos))
                } else {
                    None
                }
            })
            .sum()
    }

    fn count_part2_paths_impl(&self, current: Position) -> usize {
        if self[current] == Height::Nine {
            return 1;
        }

        DIRECT_NEIGHBORS
            .into_iter()
            .map(|neighbor| {
                if let Some(&other) = self.get(current + neighbor) {
                    if self[current].is_uphill_by_one(other) {
                        return self.count_part2_paths_impl(current + neighbor);
                    }
                }

                0
            })
            .sum()
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for height in row.iter() {
                write!(f, "{}", height)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::ops::Index<Position> for HeightMap {
    type Output = Height;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in heightmap"))
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> HeightMap {
    HeightMap {
        map: input
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::trim)
            .map(str::as_bytes)
            .map(|line| line.iter().copied().map_into().collect())
            .collect(),
    }
}

#[aoc(day10, part1)]
fn part1(height_map: &HeightMap) -> usize {
    height_map.count_part1_paths()
}

#[aoc(day10, part2)]
fn part2(height_map: &HeightMap) -> usize {
    height_map.count_part2_paths()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLES: [(&str, usize); 5] = [
        (
            "0123
1234
8765
9876",
            1,
        ),
        (
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
            2,
        ),
        (
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
            4,
        ),
        (
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
            3,
        ),
        (
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            36,
        ),
    ];

    #[test]
    fn part1_example() {
        for (example, expected) in PART1_EXAMPLES {
            assert_eq!(part1(&parse(example)), expected);
        }
    }

    const PART2_EXAMPLES: [(&str, usize); 4] = [
        (
            ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
            3,
        ),
        (
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
            13,
        ),
        (
            "012345
123456
234567
345678
4.6789
56789.",
            227,
        ),
        (
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            81,
        ),
    ];

    #[test]
    fn part2_example() {
        for (example, expected) in PART2_EXAMPLES {
            assert_eq!(part2(&parse(example)), expected);
        }
    }
}
