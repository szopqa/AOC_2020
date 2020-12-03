pub mod solution;
use super::puzzles::solution::Solution;

pub mod day_1;
pub mod day_2;
pub mod day_3;

pub fn solve_all() {
    day_1::Puzzle::solve("day_1");
    day_2::Puzzle::solve("day_2");
    day_3::Puzzle::solve("day_3");
}