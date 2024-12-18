use core::fmt;
use std::{error::Error, str::FromStr};

use fnv::FnvHashSet;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(isize, isize);

const DIRECT_NEIGHBORS: [Position; 4] = [
    Position(1, 0),
    Position(0, 1),
    Position(-1, 0),
    Position(0, -1),
];

impl Position {
    fn into_usize_tuple(self) -> (usize, usize) {
        if self.0 < 0 || self.1 < 0 {
            panic!("Cannot convert {self} to usize pair");
        }

        (self.0 as usize, self.1 as usize)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({}, {})", self.0, self.1)
        } else {
            write!(f, "{},{}", self.0, self.1)
        }
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

impl FromStr for Position {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x, y)) = s.split_once(',') else {
            return Err(format!("{s:?} does not contain a ','").into());
        };

        Ok(Self(x.trim().parse()?, y.trim().parse()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Tile {
    #[default]
    Walkable,
    Corrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    map: Array2<Tile>,
}

impl Grid {
    fn new(size: Position) -> Self {
        Grid {
            map: Array2::default((size.0 as usize, size.1 as usize)),
        }
    }

    fn grid_size(&self) -> Position {
        let mut axes = self.map.axes();
        let x = axes.next().expect("X Axis missing").len as isize;
        let y = axes.next().expect("Y Axis missing").len as isize;
        Position(x, y)
    }

    fn is_blocked(&self, pos: Position) -> bool {
        self[pos] == Tile::Corrupted
    }

    fn shape(&self) -> [usize; 2] {
        let grid_size = self.grid_size();
        [grid_size.0 as usize, grid_size.1 as usize]
    }
}

impl std::ops::Index<Position> for Grid {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        if index.0 < 0 || index.1 < 0 {
            return &Tile::Corrupted;
        }

        self.map
            .get(index.into_usize_tuple())
            .unwrap_or(&Tile::Corrupted)
    }
}

impl std::ops::IndexMut<Position> for Grid {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.map[(index.0 as usize, index.1 as usize)]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shape = self.shape();
        for col_idx in 0..shape[0] {
            for row_idx in 0..shape[1] {
                match self.map[(row_idx, col_idx)] {
                    Tile::Corrupted => write!(f, "#")?,
                    Tile::Walkable => write!(f, " ")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

mod a_star {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, VecDeque},
        usize,
    };

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct AStarNode {
        g: usize,
        h: usize,
        parent: Position,
    }

    impl AStarNode {
        const fn f(&self) -> usize {
            self.g.saturating_add(self.h)
        }
    }

    impl Default for AStarNode {
        fn default() -> Self {
            Self {
                g: usize::MAX,
                h: usize::MAX,
                parent: Position(-1, -1),
            }
        }
    }

    fn a_star_heuristic(from: Position, to: Position) -> usize {
        from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
    }

    fn trace_route(nodes: &Array2<AStarNode>, mut end_pos: Position) -> Vec<Position> {
        let mut result = VecDeque::new();
        result.push_front(end_pos);

        while let Some(node) = trace_route_exit_condition(nodes, end_pos) {
            end_pos = node.parent;
            result.push_front(end_pos);
        }

        result.into()
    }

    fn trace_route_exit_condition(
        nodes: &Array2<AStarNode>,
        end_pos: Position,
    ) -> Option<AStarNode> {
        let node = nodes[end_pos.into_usize_tuple()];
        (node.parent != end_pos).then_some(node)
    }

    pub fn a_star(grid: &Grid, start_pos: Position, target_pos: Position) -> Option<Vec<Position>> {
        if grid.is_blocked(target_pos) || grid.is_blocked(start_pos) {
            return None;
        }

        let mut closed_list: Array2<bool> = Array2::from_elem(grid.shape(), false);
        let mut nodes: Array2<AStarNode> = Array2::default(grid.shape());
        for row in 0..grid.shape()[0] {
            for col in 0..grid.shape()[1] {
                let node = &mut nodes[(row, col)];
                node.parent = Position(row as isize, col as isize);
                node.h = a_star_heuristic(node.parent, target_pos);
            }
        }

        nodes[start_pos.into_usize_tuple()] = AStarNode {
            g: 0,
            h: 0,
            parent: start_pos,
        };

        let mut open_list =
            BinaryHeap::<(Reverse<usize>, Position)>::from_iter([(Reverse(0), start_pos)]);
        while let Some((_, position)) = open_list.pop() {
            let closed = &mut closed_list[position.into_usize_tuple()];
            if *closed {
                continue;
            }

            *closed = true;

            for neighbour in DIRECT_NEIGHBORS.map(|neighbour| neighbour + position) {
                if grid.is_blocked(neighbour) || closed_list[neighbour.into_usize_tuple()] {
                    continue;
                }

                if target_pos == neighbour {
                    nodes[neighbour.into_usize_tuple()].parent = position;
                    return Some(trace_route(&nodes, target_pos));
                }

                let mut node_clone = nodes[neighbour.into_usize_tuple()].clone();
                node_clone.g = nodes[position.into_usize_tuple()].g + 1;
                if nodes[neighbour.into_usize_tuple()].f() <= node_clone.f() {
                    continue;
                }

                node_clone.parent = position;
                nodes[neighbour.into_usize_tuple()] = node_clone;
                open_list.push((Reverse(node_clone.f()), neighbour));
            }
        }

        None
    }
}

fn part1_with_grid_size(
    falling_bytes: &[Position],
    grid_size: Position,
    falling_bytes_to_take: usize,
) -> usize {
    let target = grid_size - Position(1, 1);
    let mut grid = Grid::new(grid_size);
    for &byte in falling_bytes.iter().take(falling_bytes_to_take) {
        grid[byte] = Tile::Corrupted;
    }

    // println!("{grid}");
    a_star::a_star(&grid, Position(0, 0), target)
        .expect("Could not find a path")
        .len()
        - 1
}

fn part2_with_grid_size(
    falling_bytes: &[Position],
    grid_size: Position,
    fallen_bytes: usize,
) -> Position {
    let target = grid_size - Position(1, 1);
    let mut grid = Grid::new(grid_size);
    for &byte in falling_bytes.iter().take(fallen_bytes) {
        grid[byte] = Tile::Corrupted;
    }

    let mut path = a_star::a_star(&grid, Position(0, 0), target)
        .expect("First path could not be resolved")
        .into_iter()
        .collect::<FnvHashSet<_>>();

    for i in fallen_bytes..falling_bytes.len() {
        grid[falling_bytes[i]] = Tile::Corrupted;
        if !path.contains(&falling_bytes[i]) {
            continue;
        }

        path = match a_star::a_star(&grid, Position(0, 0), target) {
            Some(p) => p.into_iter().collect(),
            None => return falling_bytes[i],
        };
    }

    unreachable!("Falling bytes did not completely block the path");
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Vec<Position>, Box<dyn Error>> {
    input.lines().map(|line| line.parse()).try_collect()
}

#[aoc(day18, part1)]
fn part1(falling_bytes: &[Position]) -> usize {
    part1_with_grid_size(falling_bytes, Position(71, 71), 1024)
}

#[aoc(day18, part2)]
fn part2(falling_bytes: &[Position]) -> Position {
    part2_with_grid_size(falling_bytes, Position(71, 71), 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1_with_grid_size(&parse(EXAMPLE).unwrap(), Position(7, 7), 12),
            22
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_with_grid_size(&parse(EXAMPLE).unwrap(), Position(7, 7), 12),
            Position(6, 1)
        );
    }
}
