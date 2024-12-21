use fnv::FnvHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone)]
struct RobotRemote {
    current_position: Position,
    controlling_remote: Option<Box<RobotRemote>>,
    cache: FnvHashMap<(Position, Position, [u8; 4]), (u64, Position)>,
}

impl RobotRemote {
    const GAP_POSITION: Position = Position(0, 0);

    fn from_initial_position(position: Position) -> Self {
        Self {
            current_position: position,
            controlling_remote: None,
            cache: FnvHashMap::default(),
        }
    }

    fn add_controller_depth(&mut self, depth: usize) {
        if depth == 0 {
            self.controlling_remote = None;
            return;
        }

        if self.controlling_remote.is_none() {
            self.controlling_remote =
                Some(Self::from_initial_position(Self::position_of_key(b'A')).into());
        }

        self.controlling_remote
            .as_mut()
            .expect("No remote after just creating one")
            .add_controller_depth(depth - 1);
    }

    fn position_of_key(key: u8) -> Position {
        match key {
            b'<' => Position(1, 0),
            b'v' => Position(1, 1),
            b'>' => Position(1, 2),
            b'^' => Position(0, 1),
            b'A' => Position(0, 2),
            _ => panic!("Key {:?} is not on robot controller", key as char),
        }
    }

    fn reset_positions(&mut self, position_of_a: Position) {
        self.current_position = position_of_a;
        if let Some(ref mut remote) = self.controlling_remote {
            remote.reset_positions(Self::position_of_key(b'A'));
        }
    }

    fn move_to_position(
        &mut self,
        target_position: Position,
        gap_position: Position,
        depth: usize,
    ) -> u64 {
        let delta = target_position - self.current_position;
        if delta == Position(0, 0) {
            return 0;
        }

        if let Some(ref mut remote) = self.controlling_remote {
            let remote_pos = remote.current_position;
            let order_of_presses =
                order_of_presses(self.current_position, target_position, gap_position);

            if let Some((cost, new_remote_pos)) =
                remote.cache.get(&(delta, remote_pos, order_of_presses))
            {
                // println!("Cache hit at depth {depth}!!");
                self.current_position = target_position;
                remote.current_position = *new_remote_pos;
                return *cost;
            }

            let mut presses = 0;
            for button in order_of_presses {
                let presses_in_direction = presses_in_direction(delta, button);
                if presses_in_direction > 0 {
                    presses += remote.move_to_position(
                        Self::position_of_key(button),
                        Self::GAP_POSITION,
                        depth + 1,
                    );
                    presses += remote.press(presses_in_direction, depth + 1, button);
                }
            }

            remote.cache.insert(
                (delta, remote_pos, order_of_presses),
                (presses, remote.current_position),
            );

            self.current_position = target_position;
            presses
        } else {
            self.current_position = target_position;
            delta.0.unsigned_abs() as u64 + delta.1.unsigned_abs() as u64
        }
    }

    fn press(&mut self, press_n_times: usize, depth: usize, _key: u8) -> u64 {
        // if depth == 2 {
        // for _ in 0..press_n_times {
        // print!("{}", key as char);
        // }
        // }

        if let Some(ref mut remote) = self.controlling_remote {
            remote.move_to_position(Self::position_of_key(b'A'), Self::GAP_POSITION, depth + 1)
                + remote.press(press_n_times, depth + 1, b'A')
        } else {
            press_n_times as u64
        }
    }
}

fn order_of_presses(from: Position, to: Position, gap: Position) -> [u8; 4] {
    if from.0 == gap.0 && to.1 == gap.1 {
        [b'v', b'^', b'<', b'>']
    } else if from.1 == gap.1 && to.0 == gap.0 {
        [b'<', b'>', b'v', b'^']
    } else {
        [b'<', b'v', b'^', b'>']
    }
}

fn presses_in_direction(delta: Position, direction: u8) -> usize {
    match direction {
        b'<' => {
            if delta.1 < 0 {
                delta.1.unsigned_abs()
            } else {
                0
            }
        }
        b'>' => {
            if delta.1 > 0 {
                delta.1.unsigned_abs()
            } else {
                0
            }
        }
        b'v' => {
            if delta.0 > 0 {
                delta.0.unsigned_abs()
            } else {
                0
            }
        }
        b'^' => {
            if delta.0 < 0 {
                delta.0.unsigned_abs()
            } else {
                0
            }
        }
        _ => unreachable!(
            "{:?} was not any of '<', 'v', '>' nor '^'",
            direction as char
        ),
    }
}

#[derive(Debug, Clone)]
struct DoorKeypad {
    controlling_remote: RobotRemote,
}

impl DoorKeypad {
    const GAP_POSITION: Position = Position(3, 0);

    fn build_part1() -> Self {
        let mut res = Self {
            controlling_remote: RobotRemote::from_initial_position(Self::position_of_key(b'A')),
        };

        res.controlling_remote.add_controller_depth(2);
        res
    }

    fn build_part2() -> Self {
        let mut res = Self {
            controlling_remote: RobotRemote::from_initial_position(Self::position_of_key(b'A')),
        };

        res.controlling_remote.add_controller_depth(25);
        res
    }

    fn position_of_key(key: u8) -> Position {
        match key {
            b'0' => Position(3, 1),
            b'1' => Position(2, 0),
            b'2' => Position(2, 1),
            b'3' => Position(2, 2),
            b'4' => Position(1, 0),
            b'5' => Position(1, 1),
            b'6' => Position(1, 2),
            b'7' => Position(0, 0),
            b'8' => Position(0, 1),
            b'9' => Position(0, 2),
            b'A' => Position(3, 2),
            _ => panic!("Key {:?} is not on door keypad", key as char),
        }
    }

    fn reset_positions(&mut self) {
        self.controlling_remote
            .reset_positions(Self::position_of_key(b'A'));
    }

    fn count_button_presses_on_top_level(&mut self, sequence: &[u8]) -> u64 {
        let mut presses = 0;
        for &key in sequence {
            presses += self.controlling_remote.move_to_position(
                Self::position_of_key(key),
                Self::GAP_POSITION,
                0,
            );
            presses += self.controlling_remote.press(1, 0, key);
        }

        self.reset_positions();
        // println!("\n    Presses: {presses}\n");
        presses
    }
}

fn sequence_number(sequence: &str) -> u64 {
    sequence
        .trim_end_matches('A')
        .trim_start_matches('0')
        .parse()
        .expect("Could not convert sequence into u64")
}

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    let mut keypad = DoorKeypad::build_part1();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|sequence| {
            // print!("Sequence {sequence}: ");
            keypad.count_button_presses_on_top_level(sequence.as_bytes())
                * sequence_number(sequence)
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    let mut keypad = DoorKeypad::build_part2();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|sequence| {
            keypad.count_button_presses_on_top_level(sequence.as_bytes())
                * sequence_number(sequence)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "029A";
    const EXAMPLE2: &str = "980A";
    const EXAMPLE3: &str = "179A";
    const EXAMPLE4: &str = "456A";
    const EXAMPLE5: &str = "379A";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(EXAMPLE1), 68 * 29);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE2), 60 * 980);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(EXAMPLE3), 68 * 179);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(EXAMPLE4), 64 * 456);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(EXAMPLE5), 64 * 379);
    }
}
