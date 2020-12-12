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

    pub fn travel_and_count_trees(&self, _x_move: usize, _y_move: usize) -> u64 { // move is always from top-left to bottom-right
        let mut _trees: u64 = 0;

        let mut _x_pos: usize = self._items.get(0).unwrap().find('.').unwrap();
        let mut _y_pos: usize = 0; // always 1st layer

        let _layer_width = self._items.get(0).unwrap().len();
        let _layers = self._items.len();

        loop {
            if _y_pos >= self._items.len() {
                break;
            }

            let _layer = self._items.get(_y_pos).unwrap();

            let _val = _layer.chars().nth(_x_pos).unwrap();

            if _val == '#' {
                _trees += 1;
            }

            _x_pos += _x_move;
            _y_pos += _y_move;

            if _x_pos > _layer_width - 1 {
                _x_pos = _x_pos - _layer_width;
            }
        }

        _trees
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let _map = Map::new(_input);
        _map.travel_and_count_trees(3, 1)
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let _map = Map::new(_input);

        vec![
            (1,1),
            (3,1),
            (5,1),
            (7,1),
            (1,2)
        ]
        .iter()
        .fold(1, |_acc, _case| {
            let _res = _map.travel_and_count_trees(_case.0, _case.1);
            _acc * _res
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_03::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 7);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 336);
    }
}