use crate::puzzles::solution::Solution;

use regex::Regex;

pub struct Puzzle {}


#[derive(Debug)]
struct PassValidator {
    _min_num: u8,
    _max_num: u8,
    _letter: char,
    _pass: String
}

impl PassValidator {
    pub fn new(_pass_line: &str) -> Self {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"^(\d+)-(\d+)\s+(\w+):\s+(\w+)").unwrap();
        }

        let _matches = REG.captures(_pass_line).unwrap();

        Self {
            _min_num: _matches.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            _max_num: _matches.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            _letter: _matches.get(3).unwrap().as_str().parse::<char>().unwrap(),
            _pass: _matches.get(4).unwrap().as_str().parse::<String>().unwrap()
        }
    }

    pub fn is_valid_part_one(&self) -> bool {
        let mut _occurrences = 0;

        for _each_letter in self._pass.chars() {
            if _each_letter == self._letter {
                _occurrences += 1;
            }
        }

        _occurrences >= self._min_num && _occurrences <= self._max_num
    }

    pub fn is_valid_part_two(&self) -> bool {
        let mut _occurrences = 0;

        let _at_min = self._pass.chars().nth(self._min_num as usize - 1).unwrap();
        let _at_max = self._pass.chars().nth(self._max_num as usize - 1).unwrap();

        (_at_min == self._letter && _at_max != self._letter) ||
            (_at_min != self._letter && _at_max == self._letter)
    }
}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = usize;
    type OutputPartTwo = usize;

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut _result: Self::OutputPartOne = 0;

        _input
            .iter()
            .map(|_pass| PassValidator::new(_pass))
            .filter(|_v| _v. is_valid_part_one())
            .collect::<Vec<PassValidator>>()
            .len()
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut _result: Self::OutputPartTwo = 0;

        _input
            .iter()
            .map(|_pass| PassValidator::new(_pass))
            .filter(|_v| _v. is_valid_part_two())
            .collect::<Vec<PassValidator>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_2::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 2);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 1);
    }
}