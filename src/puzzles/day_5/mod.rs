use crate::puzzles::solution::Solution;

#[derive(Debug, Copy, Clone)]
struct Seat {
    _pos_in_row: u8,
    _id: u64
}

impl Default for Seat {
    fn default() -> Self {
        Self {_pos_in_row: 0, _id: 0}
    }
}

impl Seat {
    pub fn new(_pos_in_row: u8, _id: u64) -> Self {
        Self {
            _pos_in_row,
            _id
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Row { 
    _num: u8,
    _seats: [Seat; 8]
}

impl Default for Row {
    fn default() -> Self {
        Self {_num: 0, _seats: [Seat::default(); 8]}
    }
}

impl Row {
    pub fn new(_num: u8) -> Self {
        let mut _seats: [Seat; 8] = [Seat::default(); 8];
        for _i in 0..8 {
            _seats[_i] = Seat {_pos_in_row: _i as u8, _id: 0}
        }
        Self {
            _num,
            _seats
        }
    }
}

#[derive(Debug)]
struct Plane {
    _seats_rows: [Row; 128]
}

impl Plane {
    pub fn new() -> Self {
        let mut _seats_rows: [Row; 128] = [Row::default(); 128];
        for _i in 0..128 {
            _seats_rows[_i] = Row::new(_i as u8);
        }

        Self {
            _seats_rows
        }
    }

    fn evaluate_pos_binary(_desc: &str) -> u8 {
        let mut _range = 0..2u8.pow(_desc.len() as u32);
        let mut _ans: u8 = 0;

        let mut _pos = 0;
        while _pos <= _desc.len() - 1 {
            match &_desc[_pos.._pos+1] {
                "R" | "B"=> {
                    _range.start = (_range.start + _range.end) / 2;
                    _ans = _range.end - 1;
                },
                "L" | "F" => {
                    _range.end = (_range.start + _range.end) / 2;
                    _ans = _range.start;
                },
                _ => panic!("Invalid value position evaluation")
            }

            _pos += 1;
        }

        _ans as u8
    }

    pub fn add_passenger(&self, _boarding_pass: &str) -> Seat {
        let _row_pos = Self::evaluate_pos_binary(&_boarding_pass[..7]);
        let _col_pos = Self::evaluate_pos_binary(&_boarding_pass[_boarding_pass.len() - 3..]);
        let _passenger_seat = Seat::new(_col_pos, _row_pos as u64 * 8 + _col_pos as u64);

        _passenger_seat
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut _plane = Plane::new();

        _input
            .iter()
            .map(|_i| _plane.add_passenger(_i))
            .max_by_key(|_s| _s._id).unwrap()._id
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_5::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "BFFFBBFRRR",
            "FFFBBBFRRR",
            "BBFFBBFRLL"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 820);
    }

    #[test]
    fn test_part_two() {
    }
}