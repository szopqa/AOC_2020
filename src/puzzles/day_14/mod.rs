use std::collections::HashMap;

use crate::puzzles::solution::Solution;

#[derive(Debug)]
enum Operation {
    Mask(String),
    Mem(usize, usize),
}

fn parse(input: &Vec<String>) -> Vec<Operation> {
    let mut memory_operations: Vec<Operation> = vec![];

    for i in input {
       match &i[0..3] {
            "mas" => {
                let mask: &str = i.split('=').collect::<Vec<&str>>().get(1).unwrap().trim();
                let mask = Operation::Mask(mask.to_string());
                memory_operations.push(mask);
            },
            "mem" => {
                let op: Vec<&str> = i.split('=').collect();
                let val = op.get(1).unwrap().trim();
                let addr = op.get(0).unwrap().trim();
                let addr = &addr[4..addr.len()-1];
                memory_operations.push(Operation::Mem(addr.parse::<usize>().unwrap(), val.parse::<usize>().unwrap()));
            },
            _ => unreachable!()
       }
    }
    
    memory_operations
}

fn apply_mask_p1(value: &usize, mask: &str) -> usize {
    let and = mask.replace("X", "1");
    let or = mask.replace("X", "0");

    value & usize::from_str_radix(and.as_str(), 2).unwrap()
        | usize::from_str_radix(or.as_str(), 2).unwrap()
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = usize;
    type OutputPartTwo = usize;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let memory_operations = parse(input);

        let mut memory: HashMap<usize, usize> = HashMap::new();
        let mut mask = "".to_string();

        for op in memory_operations {
            match op {
                Operation::Mem(addr, val) => {
                    memory.insert(addr, apply_mask_p1(&val, &mask));
                }
                Operation::Mask(m) => {
                    mask = m
                }
            }
        }

        memory.values().sum()
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_14::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 165);
    }
}