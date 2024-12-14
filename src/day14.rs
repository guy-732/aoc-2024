use std::{error::Error, str::FromStr};

use itertools::Itertools;

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

impl Position {
    fn wrap(&mut self, dimension: Position) {
        self.0 = self.0.rem_euclid(dimension.0);
        self.1 = self.1.rem_euclid(dimension.1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: Position,
    speed: Position,
}

impl FromStr for Position {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left_num, right_num) = s
            .split_once(',')
            .ok_or("Could not split ',' from position")?;

        Ok(Position(left_num.parse()?, right_num.parse()?))
    }
}

impl FromStr for Robot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.trim().split_once(' ').ok_or("Could not split on ' '")?;
        let position = position
            .strip_prefix("p=")
            .ok_or("Could not remove \"p=\" from position")?;
        let velocity = velocity
            .strip_prefix("v=")
            .ok_or("Could not remove \"v=\" from velocity")?;

        Ok(Self {
            position: position.parse()?,
            speed: velocity.parse()?,
        })
    }
}

impl Robot {
    fn position_after_n_seconds(&self, dimension: Position, n_seconds: isize) -> Position {
        let mut robot_position = self.position;
        robot_position += self.speed * n_seconds;
        robot_position.wrap(dimension);

        robot_position
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    input.lines().map(|line| line.parse()).try_collect()
}

fn part1_in_dim(robots: &[Robot], dimension: Position) -> u64 {
    let horizontal_middle = dimension.0 / 2;
    let vertical_middle = dimension.1 / 2;
    let mut quadrants = [0, 0, 0, 0];

    for robot_pos in robots
        .iter()
        .map(|robot| robot.position_after_n_seconds(dimension, 100))
    {
        if robot_pos.0 == horizontal_middle || robot_pos.1 == vertical_middle {
            continue;
        }

        if robot_pos.0 < horizontal_middle {
            if robot_pos.1 < vertical_middle {
                quadrants[0] += 1;
            } else {
                quadrants[2] += 1;
            }
        } else {
            if robot_pos.1 < vertical_middle {
                quadrants[1] += 1;
            } else {
                quadrants[3] += 1;
            }
        }
    }

    quadrants.into_iter().product()
}

#[aoc(day14, part1)]
fn part1(robots: &[Robot]) -> u64 {
    part1_in_dim(robots, Position(101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1_in_dim(&parse(EXAMPLE).unwrap(), Position(11, 7)), 12);
    }
}
