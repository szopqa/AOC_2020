pub mod solution;
use super::puzzles::solution::Solution;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;

pub fn solve_all() {
    day_1::Puzzle::solve("day_1");
    day_2::Puzzle::solve("day_2");
    day_3::Puzzle::solve("day_3");
    day_4::Puzzle::solve("day_4");
    day_5::Puzzle::solve("day_5");
}