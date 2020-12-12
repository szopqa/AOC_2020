use crate::puzzles::solution::Solution;

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        0
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_12::*;

    #[test]
    fn test_part_one() {
    }
}