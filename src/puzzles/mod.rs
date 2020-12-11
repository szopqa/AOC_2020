use std::time::{Instant};

pub mod solution;
use super::puzzles::solution::Solution;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;

fn solve_puzzle<F>(_puzzle_name: &str, _run_solution: F) -> ()
where
    F: Fn(&str) -> ()
{
    println!("\nSolving puzzle: {}", _puzzle_name);
    let now = Instant::now();
    _run_solution(_puzzle_name);
    println!("Solved in {}ms", now.elapsed().as_millis());
}

pub fn solve_all() {
    // solve_puzzle("day_1", day_1::Puzzle::solve);
    // solve_puzzle("day_2", day_2::Puzzle::solve);
    // solve_puzzle("day_3", day_3::Puzzle::solve);
    // solve_puzzle("day_4", day_4::Puzzle::solve);
    // solve_puzzle("day_5", day_5::Puzzle::solve);
    // solve_puzzle("day_6", day_6::Puzzle::solve);
    // solve_puzzle("day_7", day_7::Puzzle::solve);
    // solve_puzzle("day_8", day_8::Puzzle::solve);
    // solve_puzzle("day_9", day_9::Puzzle::solve);
    // solve_puzzle("day_10", day_10::Puzzle::solve);
    solve_puzzle("day_11", day_11::Puzzle::solve);
}