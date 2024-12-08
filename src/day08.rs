use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

type Position = (isize, isize);

fn calculate_antinode_positions(a: Position, b: Position) -> (Position, Position) {
    let delta_x = a.0 - b.0;
    let delta_y = a.1 - b.1;

    (
        (a.0 + delta_x, a.1 + delta_y),
        (b.0 - delta_x, b.1 - delta_y),
    )
}

#[derive(Debug, Clone)]
struct Map {
    antennas: FnvHashMap<char, FnvHashSet<Position>>,
    grid_height: usize,
    grid_width: usize,
}

impl Map {
    fn is_position_within(&self, position: Position) -> bool {
        if position.0 < 0 || position.1 < 0 {
            return false;
        }

        self.grid_height > position.0 as usize && self.grid_width > position.1 as usize
    }
}

#[aoc_generator(day08)]
fn parse(input: &str) -> Map {
    let mut antennas = FnvHashMap::default();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9') {
                let entry = antennas.entry(c);
                let entry: &mut FnvHashSet<Position> = entry.or_default();
                entry.insert((row as isize, col as isize));
            }
        }
    }

    Map {
        antennas,
        grid_height: input.lines().count(),
        grid_width: input.lines().next().unwrap().chars().count(),
    }
}

fn part1_do_freq(
    map: &Map,
    antenna_positions: &FnvHashSet<Position>,
    antinodes: &mut FnvHashSet<Position>,
) {
    for (&a, &b) in antenna_positions.iter().tuple_combinations() {
        let (node1, node2) = calculate_antinode_positions(a, b);

        if map.is_position_within(node1) {
            antinodes.insert(node1);
        }

        if map.is_position_within(node2) {
            antinodes.insert(node2);
        }
    }
}

fn part2_do_freq(
    map: &Map,
    antenna_positions: &FnvHashSet<Position>,
    antinodes: &mut FnvHashSet<Position>,
) {
    for (&a, &b) in antenna_positions.iter().tuple_combinations() {
        part2_all_antinodes(map, antinodes, a, b);
    }
}

fn part2_all_antinodes(map: &Map, antinodes: &mut FnvHashSet<Position>, a: Position, b: Position) {
    let delta_x = a.0 - b.0;
    let delta_y = a.1 - b.1;

    let mut position = a;
    while map.is_position_within(position) {
        antinodes.insert(position);

        position = (position.0 + delta_x, position.1 + delta_y);
    }

    position = b;
    while map.is_position_within(position) {
        antinodes.insert(position);

        position = (position.0 - delta_x, position.1 - delta_y);
    }
}

#[aoc(day08, part1)]
fn part1(map: &Map) -> usize {
    let mut antinodes = FnvHashSet::default();

    for (_frequency, antenna_positions) in map.antennas.iter() {
        part1_do_freq(map, antenna_positions, &mut antinodes);
    }

    antinodes.len()
}

#[aoc(day08, part2)]
fn part2(map: &Map) -> usize {
    let mut antinodes = FnvHashSet::default();

    for (_frequency, antenna_positions) in map.antennas.iter() {
        part2_do_freq(map, antenna_positions, &mut antinodes);
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }

    const PART2_EXAMPLE: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(PART2_EXAMPLE)), 9);
    }
}
