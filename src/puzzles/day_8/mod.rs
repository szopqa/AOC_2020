use crate::puzzles::solution::Solution;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32)
}

impl Instruction {
    fn is_nop_or_jmp(&self) -> bool {
        match *self {
            Instruction::JMP(_) | Instruction::NOP(_) => true,
            _ => false
        }
    }
}

struct Console {
    instructions: Vec<Instruction>,
    position: u32,
    last_operations_swapped_position: usize,
    pub acc: i32,
    visited_positions: Vec<u32>
}

impl Console {
    pub fn new(instructions: &Vec<String>) -> Self {
        Self {
            instructions: instructions.iter().map(|i| Self::parse_instruction(i)).collect(),
            position: 0,
            last_operations_swapped_position: 0,
            acc: 0,
            visited_positions: vec![]
        }
    }

    fn parse_instruction(instruction: &str) -> Instruction {
        let instr_data: Vec<&str> = instruction.split(' ').collect();
        let instr_name: &str = instr_data.get(0).unwrap();
        let instr_value: &str = instr_data.get(1).unwrap();
        let sign_modifier: i8 =  match &instr_value[..0] {
            "-" => -1,
            _ => 1
        };

        let instr_value = sign_modifier as i32 * instr_value.parse::<i32>().unwrap();

        match instr_name {
            "nop" => Instruction::NOP(instr_value),
            "acc" => Instruction::ACC(instr_value),
            "jmp" => Instruction::JMP(instr_value),
            _ => panic!(format!("Unknown command found {}", instr_name))
        }
    }

    fn swap_nop_jmp(&mut self, pos: usize) {
        let instr = self.instructions.get(pos).unwrap();

        let swapped = match instr {
            Instruction::NOP(val) => Instruction::JMP(*val),
            Instruction::JMP(val) => Instruction::NOP(*val),
            _ => *instr
        };

        self.instructions[pos] = swapped;
    }
    
    pub fn swap_next(&mut self) {
        if self.last_operations_swapped_position != 0 {
            self.swap_nop_jmp(self.last_operations_swapped_position);
        }

        let next_pos_to_swap = self.instructions
            .iter()
            .enumerate()
            .position(|(i, instruction)| i > self.last_operations_swapped_position && instruction.is_nop_or_jmp())
            .unwrap();
        
        self.swap_nop_jmp(next_pos_to_swap);

        self.last_operations_swapped_position = next_pos_to_swap;
    }

    pub fn detect_infinite_cycle(&mut self) -> bool {
        self.position = 0;
        self.visited_positions = vec![];
        self.acc = 0;

        let mut contains_infinite_cycle = false;
        loop {
            let instruction = match self.instructions.get(self.position as usize) {
                Some(instruction) => instruction, 
                None => { break; }
            };

            if self.visited_positions.contains(&self.position) {
                contains_infinite_cycle = true;
                break;
            }

            self.visited_positions.push(self.position);

            match instruction {
                Instruction::NOP(_) => {
                    self.position += 1;
                },
                Instruction::ACC(val) => {
                    self.acc += val;
                    self.position += 1;
                },
                Instruction::JMP(val) => {
                    self.position = (self.position as i32 + val) as u32;
                }
            }
        }

        contains_infinite_cycle 
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = i32;
    type OutputPartTwo = i32;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut console = Console::new(input);
        console.detect_infinite_cycle();

        console.acc
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut console = Console::new(input);

        while console.detect_infinite_cycle() {
            console.swap_next();
        };

        console.acc
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_8::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: i32 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 5);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: i32 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 8);
    }
}