use std::collections::HashMap;

use crate::puzzles::solution::Solution;

#[derive(Debug)]
struct Passenger {
    _answers: String
}

impl Passenger {
    pub fn new(_answers: String) -> Self {
        Self {
            _answers
        }
    }
}

#[derive(Debug)]
struct Group {
    _participants: Vec<Passenger>
}

impl Group {
    pub fn new(_group_answers: &str) -> Self {
        let mut _participants_answers: Vec<&str> = _group_answers.split(' ').collect();
        let _ = _participants_answers.pop();

        let _participants: Vec<Passenger> = _participants_answers.into_iter()
            .map(|_a| Passenger::new(_a.to_string()))
            .collect();

        Self {
            _participants
        }
    }

    pub fn get_num_of_all_yes_questions(&self) -> usize {
        let mut _unique_questions: Vec<char> = vec![];

        self._participants.iter().for_each(|_p| {
            for _each_q in _p._answers.chars() {
                if !_unique_questions.contains(&_each_q) {
                    _unique_questions.push(_each_q);
                }
            }
        });

        _unique_questions.len()
    }


    pub fn get_num_of_same_yes_questions (&self) -> usize {
        let mut _q_and_a = HashMap::new();

        self._participants.iter().for_each(|_p| {
            for _each_q in _p._answers.chars() {
                let _counter = _q_and_a.entry(_each_q).or_insert(0);
                *_counter += 1;
            }
        });

        _q_and_a.iter()
            .filter(|(_question, _all_answers)| *_all_answers == &self._participants.len())
            .map(|(_q, _)| _q)
            .collect::<Vec<&char>>()
            .len()
   }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn normalize_input(_input: Vec<Self::PuzzleInput>) -> Vec<Self::PuzzleInput> {
        let mut _normalized_input = vec![];

        let mut _line_normalized: String = "".to_string();
        for _line in _input {
            if _line != "" {
                _line_normalized.push_str(&_line);
                _line_normalized.push_str(" ");
            } else {
                _normalized_input.push(_line_normalized);
                _line_normalized = "".to_string();
            }
        }

        _normalized_input.push(_line_normalized); // last line

        _normalized_input
    }    

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut _result: Self::OutputPartOne = 0;

        _input.iter()
            .map(|_q| Group::new(_q).get_num_of_all_yes_questions())
            .fold(0, |acc, num| acc + num as u64)
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut _result: Self::OutputPartTwo = 0;

        _input.iter()
            .map(|_q| Group::new(_q).get_num_of_same_yes_questions())
            .fold(0, |acc, num| acc + num as u64)
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_6::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_one(&Puzzle::normalize_input(_input));

        // then
        assert_eq!(_res, 11);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_two(&Puzzle::normalize_input(_input));

        // then
        assert_eq!(_res, 6);
    }
}