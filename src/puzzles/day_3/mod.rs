use crate::puzzles::solution::Solution;

struct Map<'a> {
    _items: &'a Vec<String>
}

impl <'a> Map <'a> {
    pub fn new(_input: &'a Vec<String>) -> Self {
        Self {
            _items: _input
        }
    }

    pub fn travel(&self, _x_move: usize, _y_move: usize) -> () { // move is always from top-left to bottom-right
        let mut _x_pos: usize = self._items.get(0).unwrap().find('.').unwrap();
        let mut _y_pos: usize = 0; // always 1st layer

        for _each_layer in self._items {
            // println!("{}, {}", _x_pos, _y_pos);
            _x_pos += _x_move;
            _y_pos += _y_move;
        }
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u8;
    type OutputPartTwo = u8;

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let _map = Map::new(_input);
        _map.travel(3, 1);

        0
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_3::*;

    #[test]
    fn test_part_one() {
    }

    #[test]
    fn test_part_two() {
    }
}