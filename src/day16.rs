use std::{collections::BinaryHeap, fmt::Write};

use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

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
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turns(self) -> (Self, Self) {
        match self {
            Self::Up | Self::Down => (Self::Left, Self::Right),
            Self::Left | Self::Right => (Self::Up, Self::Down),
        }
    }

    fn move_from_position(&self, position: Position) -> Position {
        match self {
            Self::Up => Position(position.0 - 1, position.1),
            Self::Down => Position(position.0 + 1, position.1),
            Self::Left => Position(position.0, position.1 - 1),
            Self::Right => Position(position.0, position.1 + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Walkable,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Walkable => ' ',
            Self::Wall => '#',
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Tile>>,
    start_pos: Position,
    end_pos: Position,
}

impl Map {
    fn get(&self, index: Position) -> Option<&Tile> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.map.get(index.0 as usize)?.get(index.1 as usize)
    }

    fn dijkstra_neighbors(
        &self,
        current_position: PositionWithCost,
    ) -> impl Iterator<Item = PositionWithCost> + '_ {
        let (left, right) = current_position.direction.turns();
        [
            PositionWithCost {
                position: current_position
                    .direction
                    .move_from_position(current_position.position),
                direction: current_position.direction,
                cost: current_position.cost + 1,
            },
            PositionWithCost {
                position: current_position.position,
                direction: left,
                cost: current_position.cost + 1000,
            },
            PositionWithCost {
                position: current_position.position,
                direction: right,
                cost: current_position.cost + 1000,
            },
        ]
        .into_iter()
        .filter(|pos| {
            self.get(pos.position)
                .is_some_and(|tile| *tile == Tile::Walkable)
        })
    }
}

impl std::ops::Index<Position> for Map {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in map"))
    }
}

impl<'s> FromIterator<&'s str> for Map {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut start_pos = None;
        let mut end_pos = None;
        let map = iter
            .into_iter()
            .enumerate()
            .map(|(row_idx, line)| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(col_idx, c)| match c {
                        b'#' => Tile::Wall,
                        b'.' => Tile::Walkable,
                        b'S' => {
                            start_pos = Some(Position(row_idx as isize, col_idx as isize));
                            Tile::Walkable
                        }
                        b'E' => {
                            end_pos = Some(Position(row_idx as isize, col_idx as isize));
                            Tile::Walkable
                        }
                        _ => panic!(
                            "Invalid char '{}': was not any of '#', '.', 'S' nor 'E'",
                            *c as char
                        ),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            start_pos: start_pos.expect("Did not find 'S' in input"),
            end_pos: end_pos.expect("Did not find 'E' in input"),
        }
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Map {
    input.lines().filter(|line| !line.is_empty()).collect()
}

#[aoc(day16, part1)]
fn part1(map: &Map) -> u64 {
    let result = dijkstra(map);

    result
        .into_iter()
        .filter_map(|((key, _), value)| (key == map.end_pos).then_some(value))
        .reduce(u64::min)
        .expect("Dijkstra did not reach end_pos")
}

#[derive(Debug, Clone, Copy)]
struct PositionWithCost {
    position: Position,
    direction: Direction,
    cost: u64,
}

impl From<PositionWithCost> for (Position, Direction) {
    fn from(value: PositionWithCost) -> Self {
        (value.position, value.direction)
    }
}

impl From<PositionWithCost> for u64 {
    fn from(value: PositionWithCost) -> Self {
        value.cost
    }
}

impl From<((Position, Direction), u64)> for PositionWithCost {
    fn from(value: ((Position, Direction), u64)) -> Self {
        Self {
            position: value.0 .0,
            direction: value.0 .1,
            cost: value.1,
        }
    }
}

impl PartialEq for PositionWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PositionWithCost {}

impl PartialOrd for PositionWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PositionWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse ordering
        other.cost.cmp(&self.cost)
    }
}

fn dijkstra(map: &Map) -> FnvHashMap<(Position, Direction), u64> {
    let mut visited = FnvHashSet::<(Position, Direction)>::default();
    let mut distances = FnvHashMap::default();
    let mut queue =
        BinaryHeap::<PositionWithCost>::from([((map.start_pos, Direction::Right), 0).into()]);

    while let Some(pos) = queue.pop() {
        if visited.contains(&pos.into()) {
            continue;
        }

        visited.insert(pos.into());

        for neighbor in map.dijkstra_neighbors(pos) {
            if distances
                .get(&neighbor.into())
                .is_none_or(|&cost| cost > neighbor.into())
            {
                distances.insert(neighbor.into(), neighbor.into());
                queue.push(neighbor);
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), 7036);
        assert_eq!(part1(&parse(EXAMPLE2)), 11048);
    }
}
