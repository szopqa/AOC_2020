use std::collections::HashMap;

use crate::puzzles::solution::Solution;

fn count_distinct(inp: &Vec<u32>, memory: &mut HashMap<usize, u64>, index:usize) -> u64 {
    if index == inp.len() - 1 {
        return 1;
    }

    if let Some (memoized) = memory.get(&index) {
        return *memoized;
    }

    let mut ans = 0;
    for i in index+1..inp.len() {
        if inp.get(i).unwrap() - inp.get(index).unwrap() <= 3 {
            ans += count_distinct(inp, memory, i);
        }
    }

    memory.insert(index, ans);

    ans
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = u32;
    type OutputPartOne = u32;
    type OutputPartTwo = u64;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut sorted = input.clone();
        sorted.push(0);
        sorted.push(sorted.iter().max().unwrap() + 3); // pushing device joltage
        sorted.sort();


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
        let mut sorted = input.clone();
        sorted.push(0);
        sorted.push(sorted.iter().max().unwrap() + 3); // pushing device joltage

        sorted.sort();

        let mut memory = HashMap::new();

        count_distinct(&sorted, &mut memory, 0)
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

    #[test]
    fn test_part_two() {
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
        let _res: u64 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 19208);
    }
}