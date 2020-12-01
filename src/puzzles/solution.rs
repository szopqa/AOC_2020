use std::{
    fmt::Display,
    io::{BufReader, prelude::*},
    path::Path
};

use std::fs::File;
pub trait Solution {
    type OutputPartOne: Display;
    type OutputPartTwo: Display;

    fn read_input(filename: &Path) -> Vec<i64> {
        let f = File::open(filename)
            .expect(&format!("Could not open file at path {:?}!", filename));
        let f = BufReader::new(f);

        f.lines().filter_map(|l| l.ok()).map(|l| l.parse::<i64>().unwrap()).collect()
    }

    fn solve_part_one(_input: &Vec<i64>) -> Self::OutputPartOne;
    fn solve_part_two(_input: &Vec<i64>) -> Self::OutputPartTwo;

    fn solve(_day_name: &str) {
        let _input = Self::read_input(
            &std::env::current_dir().unwrap().join("src/puzzles").join(_day_name).join("input.txt")
        );

        let _solution_part_one = Self::solve_part_one(&_input);
        let _solution_part_two = Self::solve_part_two(&_input);

        println!("Solution part one: {}", _solution_part_one);
        println!("Solution part two: {}", _solution_part_two);
    }
}