use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Plot(u8);

impl From<u8> for Plot {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0 as char)
    }
}

impl std::fmt::Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0 as char)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

const DIRECT_NEIGHBORS: [Position; 4] = [
    Position(1, 0),
    Position(0, 1),
    Position(-1, 0),
    Position(0, -1),
];

const DIRECT_NEIGHBORS_DIR: [(Position, Direction); 4] = [
    (Position(1, 0), Direction::Down),
    (Position(0, 1), Direction::Right),
    (Position(-1, 0), Direction::Up),
    (Position(0, -1), Direction::Left),
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

#[derive(Debug, Clone)]
struct Garden {
    plots: Vec<Vec<Plot>>,
}

#[derive(Debug, Clone)]
struct VisitedList {
    visited: Vec<Vec<bool>>,
}

impl VisitedList {
    fn get(&self, index: Position) -> Option<&bool> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.visited.get(index.0 as usize)?.get(index.1 as usize)
    }

    fn get_mut(&mut self, index: Position) -> Option<&mut bool> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.visited
            .get_mut(index.0 as usize)?
            .get_mut(index.1 as usize)
    }
}

impl std::ops::Index<Position> for VisitedList {
    type Output = bool;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in visited list"))
    }
}

impl std::ops::IndexMut<Position> for VisitedList {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in visited list"))
    }
}

impl Garden {
    fn get(&self, index: Position) -> Option<&Plot> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.plots.get(index.0 as usize)?.get(index.1 as usize)
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> {
        let row_len = self.plots[0].len();
        (0..self.plots.len()).flat_map(move |row_idx| {
            (0..row_len).map(move |col_idx| Position(row_idx as isize, col_idx as isize))
        })
    }

    fn init_visited_list(&self) -> VisitedList {
        let row_length = self.plots[0].len();

        VisitedList {
            visited: vec![vec![false; row_length]; self.plots.len()],
        }
    }

    fn part1_do_region(&self, position: Position, visited: &mut VisitedList) -> Region {
        let mut region = Region {
            region_plot: self[position],
            area: 1,
            perimeter: 0,
        };

        let mut to_process = vec![position];
        while let Some(position) = to_process.pop() {
            for neighbor in DIRECT_NEIGHBORS.into_iter().map(|delta| position + delta) {
                if visited.get(neighbor).is_some_and(|visited| !visited)
                    && self[neighbor] == region.region_plot
                {
                    visited[neighbor] = true;
                    region.area += 1;
                    to_process.push(neighbor);
                } else if !self
                    .get(neighbor)
                    .is_some_and(|&plot| plot == region.region_plot)
                {
                    region.perimeter += 1;
                }
            }
        }

        region
    }

    fn part1(&self) -> u64 {
        let mut visited = self.init_visited_list();
        self.iter_positions()
            .filter_map(|position| {
                if visited[position] {
                    None
                } else {
                    visited[position] = true;
                    Some(self.part1_do_region(position, &mut visited))
                }
            })
            // .inspect(|region| println!("{region:?}"))
            .map(|region| region.fence_price())
            .sum()
    }

    fn part2_do_region(&self, position: Position, visited: &mut VisitedList) -> Region {
        let mut region = Region {
            region_plot: self[position],
            area: 1,
            perimeter: 0,
        };

        let mut sides: Vec<Side> = vec![];
        let mut to_process = vec![position];
        while let Some(position) = to_process.pop() {
            for (neighbor, direction) in DIRECT_NEIGHBORS_DIR
                .into_iter()
                .map(|(delta, direction)| (position + delta, direction))
            {
                if visited.get(neighbor).is_some_and(|visited| !visited)
                    && self[neighbor] == region.region_plot
                {
                    visited[neighbor] = true;
                    region.area += 1;
                    to_process.push(neighbor);
                } else if !self
                    .get(neighbor)
                    .is_some_and(|&plot| plot == region.region_plot)
                {
                    let side = direction.create_side(position);
                    let mut inserted = false;
                    for entry in sides.iter_mut() {
                        if entry.try_merge(side).is_none() {
                            inserted = true;
                            break;
                        }
                    }

                    if !inserted {
                        sides.push(side);
                    }
                }
            }
        }

        let true_sides = Side::final_merge(sides);
        region.perimeter = true_sides.len() as u64;
        // dbg!((region, true_sides));
        region
    }

    fn part2(&self) -> u64 {
        let mut visited = self.init_visited_list();
        self.iter_positions()
            .filter_map(|position| {
                if visited[position] {
                    None
                } else {
                    visited[position] = true;
                    Some(self.part2_do_region(position, &mut visited))
                }
            })
            // .inspect(|region| println!("{region:?}"))
            .map(|region| region.fence_price())
            .sum()
    }
}

impl std::ops::Index<Position> for Garden {
    type Output = Plot;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in garden"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Region {
    region_plot: Plot,
    area: u64,
    perimeter: u64,
}

impl Region {
    fn fence_price(&self) -> u64 {
        self.area * self.perimeter
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
    fn create_side(self, position: Position) -> Side {
        match self {
            Self::Down | Self::Up => Side {
                fence_side: self,
                alignment: position.0 as usize,
                bounds: (position.1 as usize, position.1 as usize),
            },
            Self::Left | Self::Right => Side {
                fence_side: self,
                alignment: position.1 as usize,
                bounds: (position.0 as usize, position.0 as usize),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Side {
    fence_side: Direction,
    alignment: usize,
    bounds: (usize, usize),
}

impl Side {
    fn try_merge(&mut self, other: Side) -> Option<Side> {
        if self.fence_side == other.fence_side && self.alignment == other.alignment {
            if self.start_can_merge_with(&other) || self.end_can_merge_with(&other) {
                self.bounds.0 = self.bounds.0.min(other.bounds.0);
                self.bounds.1 = self.bounds.1.max(other.bounds.1);
                return None;
            }
        }

        Some(other)
    }

    fn start_can_merge_with(&self, other: &Side) -> bool {
        let start = self.bounds.0;
        start + 1 >= other.bounds.0 && start <= other.bounds.1 + 1
    }

    fn end_can_merge_with(&self, other: &Side) -> bool {
        let end = self.bounds.1;
        end + 1 >= other.bounds.0 && end <= other.bounds.1 + 1
    }

    fn final_merge(mut sides: Vec<Self>) -> Vec<Side> {
        let mut changed = true;
        while changed {
            changed = false;

            let iter = sides.into_iter();
            sides = vec![];
            for side in iter {
                let mut inserted = false;
                for entry in sides.iter_mut() {
                    if entry.try_merge(side).is_none() {
                        inserted = true;
                        changed = true;
                        break;
                    }
                }

                if !inserted {
                    sides.push(side);
                }
            }
        }

        sides
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Garden {
    Garden {
        plots: input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .map(|b| Plot::from(*b))
                    .collect()
            })
            .collect(),
    }
}

#[aoc(day12, part1)]
fn part1(input: &Garden) -> u64 {
    input.part1()
}

#[aoc(day12, part2)]
fn part2(input: &Garden) -> u64 {
    input.part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE1)), 140);
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE1)), 80);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE2)), 436);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse(EXAMPLE3)), 1206);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2(&parse(EXAMPLE4)), 236);
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2(&parse(EXAMPLE5)), 368);
    }
}
