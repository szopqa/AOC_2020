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
    pub acc: i32,
    last_operations_swapped_position: usize,
}

impl Console {
    pub fn new(instructions: &Vec<String>) -> Self {
        Self {
            instructions: instructions.iter().map(|i| Self::parse_instruction(i)).collect(),
            acc: 0,
            last_operations_swapped_position: 0,
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
            self.swap_nop_jmp(self.last_operations_swapped_position); // swapping back previous instruction
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
        let mut visited_positions: Vec<usize> = vec![];
        let mut position: usize = 0;
        self.acc = 0;

        let mut contains_infinite_cycle = false;
        loop {
            let instruction = match self.instructions.get(position) {
                Some(instruction) => instruction, 
                None => { break; }
            };

            if visited_positions.contains(&position) {
                contains_infinite_cycle = true;
                break;
            }

            visited_positions.push(position);

            match instruction {
                Instruction::NOP(_) => {
                    position += 1;
                },
                Instruction::ACC(val) => {
                    self.acc += val;
                    position += 1;
                },
                Instruction::JMP(val) => {
                    position = (position as i32 + val) as usize;
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
    use crate::puzzles::day_08::*;

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