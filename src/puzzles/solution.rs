use std::{
    fmt::Display,
    io::{BufReader, prelude::*},
    path::Path,
    fs::File
};

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

    fn solve(_day_name: &str) 
        where 
            <Self::PuzzleInput as std::str::FromStr>::Err: std::fmt::Debug 
    {
        println!("\nSolving puzzle for {}", _day_name);

        let _input: Vec<Self::PuzzleInput> = Self::read_input(
            &std::env::current_dir().unwrap().join("src/puzzles").join(_day_name).join("input.txt")
        );

        let _solution_part_one = Self::solve_part_one(&_input);
        let _solution_part_two = Self::solve_part_two(&_input);

        println!("Solution part one: {}", _solution_part_one);
        println!("Solution part two: {}", _solution_part_two);
    }
}