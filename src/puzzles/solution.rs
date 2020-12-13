use std::time::{Instant};

use std::{
    fmt::Display,
    io::{BufReader, prelude::*},
    path::Path,
    fs::File
};

pub struct PuzzleResult {
    puzzle_name: String,
    part_one_time_ms: String,
    part_two_time_ms: String,
    solution_part_one: String,
    solution_part_two: String
}

impl PuzzleResult {
    pub fn new(puzzle_name: String, part_one_time: String, part_two_time: String, solution_p1: String, solution_p2: String) -> Self {
        Self {
            puzzle_name,
            part_one_time_ms: part_one_time,
            part_two_time_ms: part_two_time,
            solution_part_one: solution_p1,
            solution_part_two: solution_p2
        }
    }

    pub fn show_results(&self) {
        println!("\nResults for {}", self.puzzle_name);
        println!("  * Part one:");
        println!("       Result: {}", self.solution_part_one);
        println!("       Execution time: {}ms", self.part_one_time_ms);
        println!("  * Part two:");
        println!("       Result: {}", self.solution_part_two);
        println!("       Execution time: {}ms", self.part_two_time_ms);
    }
}

pub trait Solution {
    type PuzzleInput: std::str::FromStr + std::fmt::Debug;

    type OutputPartOne: Display;
    type OutputPartTwo: Display;

    // should be overwritten if some extra actions are required on input
    fn normalize_input(_input: Vec<Self::PuzzleInput>) -> Vec<Self::PuzzleInput> {
        _input
    }

    fn read_input(filename: &Path) -> Vec<Self::PuzzleInput> 
        where 
            <Self::PuzzleInput as std::str::FromStr>::Err: std::fmt::Debug 
    {
        let f = File::open(filename)
            .expect(&format!("Could not open file at path {:?}!", filename));
        let f = BufReader::new(f);

        Self::normalize_input(
            f.lines()
                .filter_map(|l| l.ok())
                .map(|l| l.parse::<Self::PuzzleInput>().unwrap())
                .collect()
        )
    }

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne;
    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo;

    fn solve(_day_name: &str) -> PuzzleResult
        where 
            <Self::PuzzleInput as std::str::FromStr>::Err: std::fmt::Debug 
    {
        let _input: Vec<Self::PuzzleInput> = Self::read_input(
            &std::env::current_dir().unwrap().join("src/puzzles").join(_day_name).join("input.txt")
        );

        let now = Instant::now();
        let solution_part_one = Self::solve_part_one(&_input);
        let p1_time = now.elapsed().as_millis();

        let now = Instant::now();
        let solution_part_two = Self::solve_part_two(&_input);
        let p2_time = now.elapsed().as_millis();

        PuzzleResult::new(
            (*_day_name).to_string(), 
            p1_time.to_string(), 
            p2_time.to_string(), 
            solution_part_one.to_string(), 
            solution_part_two.to_string() 
        )
    }
}