use std::collections::VecDeque;

use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(isize, isize);

const DIRECT_NEIGHBORS: [Position; 4] = [
    Position(1, 0),
    Position(0, 1),
    Position(-1, 0),
    Position(0, -1),
];

impl Position {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u64
    }

    fn iter_positions_within(
        &self,
        max_distance: usize,
    ) -> impl IntoIterator<Item = (Position, u64)> + '_ {
        let max_isize = max_distance as isize;
        ((self.0 - max_isize)..=(self.0 + max_isize))
            .flat_map(move |x_dist| {
                ((self.1 - max_isize)..=(self.1 + max_isize))
                    .map(move |y_dist| Position(x_dist, y_dist))
            })
            .map(|position| (position, self.manhattan_distance(&position)))
            .filter(move |(_, dist)| (*dist as usize) <= max_distance)
    }
}

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

impl std::ops::Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::MulAssign<isize> for Position {
    fn mul_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Blocked,
    Walkable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    start_pos: Position,
    end_pos: Position,
    map: Vec<Vec<Tile>>,
}

impl Grid {
    fn neighbours(&self, pos: Position) -> impl IntoIterator<Item = Position> + '_ {
        DIRECT_NEIGHBORS
            .into_iter()
            .map(move |delta| delta + pos)
            .filter(|pos| self[*pos] == Tile::Walkable)
    }

    fn costs_to_end(&self) -> FnvHashMap<Position, u64> {
        let mut costs = FnvHashMap::default();
        let mut queue = VecDeque::new();
        queue.push_back((self.end_pos, 0));

        while let Some((pos, cost)) = queue.pop_front() {
            costs.insert(pos, cost);
            for neighbour in self.neighbours(pos) {
                if costs.contains_key(&neighbour) {
                    continue;
                }

                queue.push_back((neighbour, cost + 1));
            }
        }

        costs
    }
}

impl std::ops::Index<Position> for Grid {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        if index.0 < 0 || index.1 < 0 {
            return &Tile::Blocked;
        }

        self.map
            .get(index.0 as usize)
            .and_then(|row| row.get(index.1 as usize))
            .unwrap_or(&Tile::Blocked)
    }
}

fn cheat_deltas(cheat_duration: usize) -> Vec<Position> {
    let mut result = vec![];
    let mut visited = FnvHashSet::default();
    let mut queue = VecDeque::new();

    visited.insert(Position(0, 0));
    queue.push_back((Position(0, 0), 0));

    while let Some((position, distance)) = queue.pop_front() {
        if distance >= cheat_duration {
            result.push(position);
            continue;
        }

        for delta in DIRECT_NEIGHBORS {
            let delta = delta + position;
            if visited.insert(delta) {
                queue.push_back((delta, distance + 1));
            }
        }
    }

    result
}

fn count_cheats_part1<F>(grid: &Grid, mut accept_cheat: F) -> u64
where
    F: FnMut(u64) -> bool,
{
    let normal_costs = grid.costs_to_end();
    let deltas = cheat_deltas(2);
    let mut accepted_cheats = 0;
    for (&start_cheat_at, &cost_from_start) in normal_costs.iter() {
        for &delta in deltas.iter() {
            let cheat_ends = start_cheat_at + delta;
            if let Some(&cost_from_end) = normal_costs.get(&cheat_ends) {
                if cost_from_end > cost_from_start {
                    continue;
                }

                if accept_cheat(cost_from_start.saturating_sub(cost_from_end + 2)) {
                    accepted_cheats += 1;
                }
            }
        }
    }

    accepted_cheats
}

fn find_cheats_part2<F>(
    accept_cheat: &mut F,
    cheat_duration: usize,
    from: Position,
    from_cost: u64,
    costs: &FnvHashMap<Position, u64>,
) -> u64
where
    F: FnMut(u64) -> bool,
{
    let mut accepted = 0;
    for (end, covered) in from.iter_positions_within(cheat_duration) {
        if let Some(end_cost) = costs.get(&end) {
            if *end_cost > from_cost {
                continue;
            }

            let saved = from_cost.saturating_sub(*end_cost + covered);
            if accept_cheat(saved) {
                accepted += 1;
            }
        }
    }

    accepted
}

fn count_cheats_part2<F>(grid: &Grid, mut accept_cheat: F, cheat_duration: usize) -> u64
where
    F: FnMut(u64) -> bool,
{
    let normal_costs = grid.costs_to_end();
    let mut accepted_cheats = 0;
    for (&start_cheat_at, &cost_from_start) in normal_costs.iter() {
        accepted_cheats += find_cheats_part2(
            &mut accept_cheat,
            cheat_duration,
            start_cheat_at,
            cost_from_start,
            &normal_costs,
        );
    }

    accepted_cheats
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Grid {
    let mut start_pos = None;
    let mut end_pos = None;
    let mut map = vec![];
    for (row_idx, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        let mut row = vec![];
        for (col_idx, c) in line.trim().as_bytes().iter().enumerate() {
            match c {
                b'#' => row.push(Tile::Blocked),
                b'.' | b' ' => row.push(Tile::Walkable),
                b'S' => {
                    start_pos = Some(Position(row_idx as isize, col_idx as isize));
                    row.push(Tile::Walkable);
                }
                b'E' => {
                    end_pos = Some(Position(row_idx as isize, col_idx as isize));
                    row.push(Tile::Walkable);
                }
                _ => panic!("{:?} was not any of '#', '.', ' ', 'S' or 'E'", *c as char),
            }
        }

        map.push(row);
    }

    Grid {
        start_pos: start_pos.expect("Start not found"),
        end_pos: end_pos.expect("End not found"),
        map,
    }
}

#[aoc(day20, part1)]
fn part1(grid: &Grid) -> u64 {
    count_cheats_part1(grid, |picoseconds_saved| picoseconds_saved >= 100)
}

#[aoc(day20, part2)]
fn part2(grid: &Grid) -> u64 {
    count_cheats_part2(grid, |picoseconds_saved| picoseconds_saved >= 100, 20)
}

#[cfg(test)]
mod tests {
    use fnv::FnvHashMap;

    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    const EXAMPLE1_CHEATS: [(u64, u64); 11] = [
        (2, 14),
        (4, 14),
        (6, 2),
        (8, 4),
        (10, 2),
        (12, 3),
        (20, 1),
        (36, 1),
        (38, 1),
        (40, 1),
        (64, 1),
    ];

    #[test]
    fn part1_example() {
        let grid = parse(EXAMPLE);
        let expected_cheats: FnvHashMap<u64, u64> = EXAMPLE1_CHEATS.into_iter().collect();
        let mut cheats_found = FnvHashMap::default();

        assert_eq!(
            count_cheats_part1(&grid, |saved| {
                if saved == 0 {
                    return false;
                }

                *cheats_found.entry(saved).or_insert(0) += 1;
                true
            }),
            expected_cheats.values().copied().sum()
        );

        assert_eq!(cheats_found, expected_cheats);
    }

    const EXAMPLE2_CHEATS: [(u64, u64); 14] = [
        (50, 32),
        (52, 31),
        (54, 29),
        (56, 39),
        (58, 25),
        (60, 23),
        (62, 20),
        (64, 19),
        (66, 12),
        (68, 14),
        (70, 12),
        (72, 22),
        (74, 4),
        (76, 3),
    ];

    #[test]
    fn part2_example() {
        let grid = parse(EXAMPLE);
        let expected_cheats: FnvHashMap<u64, u64> = EXAMPLE2_CHEATS.into_iter().collect();
        let mut cheats_found = FnvHashMap::default();

        assert_eq!(
            count_cheats_part2(
                &grid,
                |saved| {
                    if saved < 50 {
                        return false;
                    }

                    *cheats_found.entry(saved).or_insert(0) += 1;
                    true
                },
                20,
            ),
            expected_cheats.values().copied().sum()
        );

        assert_eq!(cheats_found, expected_cheats);
    }
}
