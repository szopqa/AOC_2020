use std::thread::{spawn, JoinHandle};

pub mod solution;
use super::puzzles::solution::{Solution, PuzzleResult};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;

fn solve_puzzle<F>(_puzzle_name: &'static str, _run_solution: F) -> JoinHandle<()>
where
    F: Fn(&str) -> PuzzleResult + std::marker::Sync + Send + 'static
{
    spawn(move || {
        println!("Starting solution for {}", _puzzle_name);
        _run_solution(_puzzle_name).show_results();
    })
}

pub fn solve_all() {
    let handles : Vec<JoinHandle<()>> = vec![
        solve_puzzle("day_01", day_01::Puzzle::solve),
        solve_puzzle("day_02", day_02::Puzzle::solve),
        solve_puzzle("day_03", day_03::Puzzle::solve),
        solve_puzzle("day_04", day_04::Puzzle::solve),
        solve_puzzle("day_05", day_05::Puzzle::solve),
        solve_puzzle("day_06", day_06::Puzzle::solve),
        solve_puzzle("day_07", day_07::Puzzle::solve),
        solve_puzzle("day_08", day_08::Puzzle::solve),
        solve_puzzle("day_09", day_09::Puzzle::solve),
        solve_puzzle("day_10", day_10::Puzzle::solve),
        solve_puzzle("day_11", day_11::Puzzle::solve),
        solve_puzzle("day_12", day_12::Puzzle::solve),
        solve_puzzle("day_13", day_13::Puzzle::solve),
    ];

    for handle in handles {
        handle.join().unwrap();
    }
}