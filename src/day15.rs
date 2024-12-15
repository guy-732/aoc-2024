use std::fmt::Write;

use fnv::FnvHashMap;

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
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn move_from_position(&self, position: Position) -> Position {
        match self {
            Self::Up => Position(position.0 - 1, position.1),
            Self::Down => Position(position.0 + 1, position.1),
            Self::Left => Position(position.0, position.1 - 1),
            Self::Right => Position(position.0, position.1 + 1),
        }
    }
}

impl TryFrom<u8> for Move {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            b'v' => Ok(Self::Down),
            b'^' => Ok(Self::Up),
            _ => Err("u8's corresponding ASCII char was not any of 'v' '<' '^' or '>'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    FreeSpace,
    Box,
    BoxLeft,
    BoxRight,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Box => 'O',
                Self::FreeSpace => '.',
                Self::Wall => '#',
                Self::BoxLeft => '[',
                Self::BoxRight => ']',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Tile>>,
    robot_position: Position,
}

impl Map {
    fn sum_box_gps(&self) -> u64 {
        let mut sum = 0;
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Box | Tile::BoxLeft) {
                    sum += (row_idx * 100 + col_idx) as u64;
                }
            }
        }

        sum
    }

    fn perform_move_part2(&mut self, mv: Move) {
        let target_pos = mv.move_from_position(self.robot_position);
        match self[target_pos] {
            Tile::FreeSpace => self.robot_position = target_pos,
            Tile::Wall => (),
            Tile::BoxLeft | Tile::BoxRight => match mv {
                Move::Down | Move::Up => {
                    let mut moves_to_execute = FnvHashMap::default();
                    if self.try_push_part2_vertical(mv, target_pos, &mut moves_to_execute) {
                        for (pos, tile) in moves_to_execute {
                            self[pos] = tile;
                        }

                        self.robot_position = target_pos;
                    }
                }
                Move::Left | Move::Right => {
                    if self.push_part2_horizontal(mv, target_pos) {
                        self[target_pos] = Tile::FreeSpace;
                        self.robot_position = target_pos;
                    }
                }
            },
            other => panic!("Tile '{other}' encountered in part 2"),
        }
    }

    fn try_push_part2_vertical(
        &mut self,
        mv: Move,
        block_pos: Position,
        moves_to_execute: &mut FnvHashMap<Position, Tile>,
    ) -> bool {
        let target_pos = mv.move_from_position(block_pos);

        todo!()
    }

    fn push_part2_horizontal(&mut self, mv: Move, block_pos: Position) -> bool {
        let target_pos = mv.move_from_position(block_pos);
        match self[target_pos] {
            Tile::FreeSpace => {
                self[target_pos] = self[block_pos];
                true
            }
            Tile::Wall => false,
            Tile::BoxLeft | Tile::BoxRight => {
                if self.push_part2_horizontal(mv, target_pos) {
                    self[target_pos] = self[block_pos];
                    true
                } else {
                    false
                }
            }
            other => panic!("Tile '{other}' encountered in part 2"),
        }
    }

    fn perform_move_part1(&mut self, mv: Move) {
        let target_pos = mv.move_from_position(self.robot_position);
        match self[target_pos] {
            Tile::FreeSpace => {
                self.robot_position = target_pos;
            }
            Tile::Wall => (),
            Tile::Box => {
                if self.push_block_part1(mv, target_pos) {
                    self[target_pos] = Tile::FreeSpace;
                    self.robot_position = target_pos;
                }
            }
            other => panic!("Tile '{other}' encountered in part 1"),
        }
    }

    fn push_block_part1(&mut self, mv: Move, block_pos: Position) -> bool {
        let target_pos = mv.move_from_position(block_pos);
        match self[target_pos] {
            Tile::FreeSpace => {
                self[target_pos] = Tile::Box;
                true
            }
            Tile::Wall => false,
            Tile::Box => self.push_block_part1(mv, target_pos),
            other => panic!("Tile '{other}' encountered in part 1"),
        }
    }

    fn get(&self, index: Position) -> Option<&Tile> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.map.get(index.0 as usize)?.get(index.1 as usize)
    }

    fn get_mut(&mut self, index: Position) -> Option<&mut Tile> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }

        self.map
            .get_mut(index.0 as usize)?
            .get_mut(index.1 as usize)
    }

    fn from_iter_part2<'s, I: IntoIterator<Item = &'s str>>(iter: I) -> Self {
        let mut bot_position = None;
        let map = iter
            .into_iter()
            .enumerate()
            .map(|(row_idx, line)| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .flat_map(|(col_idx, c)| match c {
                        b'#' => [Tile::Wall, Tile::Wall],
                        b'.' => [Tile::FreeSpace, Tile::FreeSpace],
                        b'O' => [Tile::BoxLeft, Tile::BoxRight],
                        b'@' => {
                            bot_position = Some(Position(row_idx as isize, (col_idx * 2) as isize));
                            [Tile::FreeSpace, Tile::FreeSpace]
                        }
                        _ => panic!(
                            "Invalid char '{}': was not any of '#', '.', 'O' nor '@'",
                            *c as char
                        ),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            robot_position: bot_position.expect("Did not find '@' in input"),
        }
    }
}

impl std::ops::Index<Position> for Map {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in map"))
    }
}

impl std::ops::IndexMut<Position> for Map {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Could not index position {index} in map"))
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..(self.map.len() as isize) {
            for col in 0..(self.map[row as usize].len() as isize) {
                if Position(row, col) == self.robot_position {
                    f.write_char('@')?;
                    continue;
                }

                write!(f, "{}", self[Position(row, col)])?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<'s> FromIterator<&'s str> for Map {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut bot_position = None;
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
                        b'.' => Tile::FreeSpace,
                        b'O' => Tile::Box,
                        b'@' => {
                            bot_position = Some(Position(row_idx as isize, col_idx as isize));
                            Tile::FreeSpace
                        }
                        _ => panic!(
                            "Invalid char '{}': was not any of '#', '.', 'O' nor '@'",
                            *c as char
                        ),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            robot_position: bot_position.expect("Did not find '@' in input"),
        }
    }
}

fn parse_part1(input: &str) -> (Map, Vec<Move>) {
    let mut lines = input.lines();
    let map = lines.by_ref().take_while(|line| !line.is_empty()).collect();

    (
        map,
        lines
            .flat_map(|line| {
                line.as_bytes()
                    .iter()
                    .filter_map(|c| match Move::try_from(*c) {
                        Ok(mv) => Some(mv),
                        Err(_) => None,
                    })
            })
            .collect(),
    )
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let (mut map, moves) = parse_part1(input);
    for mv in moves {
        map.perform_move_part1(mv);
        // println!("Move {mv:?}:");
        // println!("{}", &map);
    }

    map.sum_box_gps()
}

fn parse_part2(input: &str) -> (Map, Vec<Move>) {
    let mut lines = input.lines();
    let map = Map::from_iter_part2(lines.by_ref().take_while(|line| !line.is_empty()));

    (
        map,
        lines
            .flat_map(|line| {
                line.as_bytes()
                    .iter()
                    .filter_map(|c| match Move::try_from(*c) {
                        Ok(mv) => Some(mv),
                        Err(_) => None,
                    })
            })
            .collect(),
    )
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let (mut map, moves) = parse_part2(input);
    for mv in moves {
        map.perform_move_part2(mv);
    }

    println!("{}", &map);
    map.sum_box_gps()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), 2028);
        assert_eq!(part1(EXAMPLE2), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 9021);
    }
}
