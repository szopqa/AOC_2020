use std::collections::HashMap;

use crate::puzzles::solution::Solution;

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = u32;
    type OutputPartOne = u32;
    type OutputPartTwo = u32;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut sorted = input.clone();
        sorted.sort();

        sorted.push(sorted.iter().max().unwrap() + 3); // pushing device joltage

        let mut last_out_joltage = 0;
        let mut differences_counter: HashMap<u32, u32> = HashMap::new();

        for elem in &sorted {
            let diff = elem - last_out_joltage;
            let counter = differences_counter.entry(diff).or_insert(0);
            *counter += 1;
            
            last_out_joltage = *elem;
        }

        differences_counter.get(&1).unwrap() * differences_counter.get(&3).unwrap()
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_10::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ];

        // when
        let _res: u32 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 220);
    }
}