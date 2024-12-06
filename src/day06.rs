use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Walkable,
    Obstacle,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => Self::Walkable,
            '#' => Self::Obstacle,
            _ => panic!("Invalid tile: {value:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        };
    }

    fn translate_pos(self, position: Position) -> Option<Position> {
        Some(match self {
            Self::Up => (position.0.checked_sub(1)?, position.1),
            Self::Down => (position.0 + 1, position.1),
            Self::Left => (position.0, position.1.checked_sub(1)?),
            Self::Right => (position.0, position.1 + 1),
        })
    }
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct MappedArea {
    map: Vec<Vec<Tile>>,
    guard_start_pos: Position,
}

#[aoc_generator(day06)]
fn parse(input: &str) -> MappedArea {
    let mut guard_start_pos = (0, 0);
    let map = input
        .lines()
        .filter(|&line| !line.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '^' {
                        guard_start_pos = (row, col);
                    }

                    Tile::from(c)
                })
                .collect_vec()
        })
        .collect_vec();

    MappedArea {
        map,
        guard_start_pos,
    }
}

fn perform_walk(map: &MappedArea) -> (FnvHashSet<Position>, bool) {
    let mut current_position = map.guard_start_pos;
    let mut guard_direction = Direction::Up;
    let mut walked =
        FnvHashMap::from_iter(std::iter::once((current_position, vec![guard_direction])));

    let looping = loop {
        match guard_direction
            .translate_pos(current_position)
            .and_then(|new_pos| Some((new_pos, *map.map.get(new_pos.0)?.get(new_pos.1)?)))
        {
            None => {
                break false;
            }
            Some((new_pos, tile)) => {
                if tile == Tile::Obstacle {
                    guard_direction.turn_right();
                    continue;
                }

                current_position = new_pos;
                let directions = walked.entry(new_pos).or_insert_with(|| vec![]);
                if directions.contains(&guard_direction) {
                    // looping
                    break true;
                }

                directions.push(guard_direction);
            }
        }
    };

    (walked.keys().copied().collect(), looping)
}

#[aoc(day06, part1)]
fn part1(input: &MappedArea) -> usize {
    let (walked, looping) = perform_walk(input);
    assert_eq!(looping, false);
    walked.len()
}

#[aoc(day06, part2)]
fn part2(input: &MappedArea) -> usize {
    let walked = {
        let (mut walked, looping) = perform_walk(input);
        assert_eq!(looping, false);
        walked.remove(&input.guard_start_pos);
        walked
    };

    walked
        .into_par_iter()
        .filter(|&position| {
            let mut map = input.clone();
            map.map[position.0][position.1] = Tile::Obstacle;

            let (_, looping) = perform_walk(&map);
            looping
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
