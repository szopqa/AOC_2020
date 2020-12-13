use crate::puzzles::solution::Solution;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W
}

#[derive(Debug, PartialEq)]
enum Turn {
    R,
    L
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        fn parse(val: &str) -> i32 {
            val.parse::<i32>().unwrap()
        }

        match &input[0..1] {
            "N"  => Ok(Action::N(parse(&input[1..]))),
            "S"  => Ok(Action::S(parse(&input[1..]))),
            "E"  => Ok(Action::E(parse(&input[1..]))),
            "W" => Ok(Action::W(parse(&input[1..]))),
            "L" => Ok(Action::L(parse(&input[1..]))),
            "R" => Ok(Action::R(parse(&input[1..]))),
            "F" => Ok(Action::F(parse(&input[1..]))),
            _      => Err(()),
        }
    }
}

struct Waypoint {
    y_pos: i32,
    x_pos: i32,
}

pub struct Ferry {
    y_pos: i32,
    x_pos: i32,
    last_dir: Dir,
    waypoint: Waypoint
}

impl Ferry {
    pub fn new() -> Self {
        Self {
            y_pos: 0,
            x_pos: 0,
            last_dir: Dir::E,
            waypoint: Waypoint {
                x_pos: 10,
                y_pos: 1
            }
        }
    }

    fn change_ferry_pos(&mut self, dir: Dir, change_val: i32) {
        let pos = match dir {
            Dir::N | Dir::S => &mut self.y_pos,
            Dir::W | Dir::E => &mut self.x_pos
        };
        let change_val = if dir == Dir::S || dir == Dir::W {-1 * change_val} else {change_val};
        *pos += change_val;
    }

    fn change_waypoint_pos(&mut self, dir: Dir, change_val: i32) {
        let pos = match dir {
            Dir::N | Dir::S => &mut self.waypoint.y_pos,
            Dir::W | Dir::E => &mut self.waypoint.x_pos
        };
        let change_val = if dir == Dir::S || dir == Dir::W {-1 * change_val} else {change_val};
        *pos += change_val;
    }

    fn change_angle(&mut self, turn: Turn, angle: i32) {
        let directions = vec![Dir::N, Dir::E, Dir::S, Dir::W];
        let shift = (angle / 90) % 4;

        let curr = directions.iter().position(|p| *p == self.last_dir).unwrap() as i32;

        let new_pos = if turn == Turn::R {(curr + shift)% 4} else {
            let mut pos = (curr - shift) % 4;
            if pos < 0 {
                pos = directions.len() as i32 + pos;
            }
            pos
        };

        self.last_dir = *directions.get(new_pos as usize).unwrap();
    }

    fn rotate_by_90(&mut self, turn: &Turn) {
        match *turn {
            Turn::R => {
                let x = self.waypoint.x_pos;
                self.waypoint.x_pos = self.waypoint.y_pos;
                self.waypoint.y_pos = -x;
            }
            Turn::L => {
                let y = self.waypoint.y_pos;
                self.waypoint.y_pos = self.waypoint.x_pos;
                self.waypoint.x_pos = -y;
            }
        }
    }

    fn rotate_waypoint(&mut self, turn: Turn, angle: i32) {
        let shift = (angle / 90) % 4;

        for _ in 0..shift {
            self.rotate_by_90(&turn)
        }        
    }

    fn move_next(&mut self, action: Action) {
        match action {
            Action::N(val) => {
                self.change_ferry_pos(Dir::N, val);
            }
            Action::S(val) => {
                self.change_ferry_pos(Dir::S, val);
            }
            Action::E(val) => {
                self.change_ferry_pos(Dir::E, val);
            }
            Action::W(val) => {
                self.change_ferry_pos(Dir::W, val);
            }
            Action::L(val) => {
                self.change_angle(Turn::L, val);
            }
            Action::R(val) => {
                self.change_angle(Turn::R, val);
            }
            Action::F(val) => {
                let current = self.last_dir;
                self.change_ferry_pos(current, val);
            }
        }
    }

    fn move_next_with_waypoint(&mut self, action: Action) {
        match action {
            Action::N(val) => {
                self.change_waypoint_pos(Dir::N, val);
            }
            Action::S(val) => {
                self.change_waypoint_pos(Dir::S, val);
            }
            Action::E(val) => {
                self.change_waypoint_pos(Dir::E, val);
            }
            Action::W(val) => {
                self.change_waypoint_pos(Dir::W, val);
            }
            Action::L(val) => {
                self.rotate_waypoint(Turn::L, val);
            }
            Action::R(val) => {
                self.rotate_waypoint(Turn::R, val);
            }
            Action::F(val) => {
                let x = self.waypoint.x_pos * val;
                let y = self.waypoint.y_pos * val;

                self.x_pos += x;
                self.y_pos += y;
            }
        }
    }

    fn get_distance(&self) -> i32 {
        self.x_pos.abs() + self.y_pos.abs()
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = i32;
    type OutputPartTwo = i32;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut ferry = Ferry::new();
        for c in input {
            ferry.move_next(Action::from_str(c).unwrap());
        }
        ferry.get_distance()
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut ferry = Ferry::new();
        for c in input {
            ferry.move_next_with_waypoint(Action::from_str(c).unwrap());
            // ferry.get_pos();
        }
        ferry.get_distance()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_12::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: i32 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 25);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: i32 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 286);
    }
}