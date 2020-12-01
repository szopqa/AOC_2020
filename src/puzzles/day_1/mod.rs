use crate::puzzles::solution::Solution;

pub struct Puzzle {}

const EXPECTED_SUM: i64 = 2020;

impl Solution for Puzzle {
    type OutputPartOne = i64;
    type OutputPartTwo = i64;

    fn solve_part_one(_input: &Vec<i64>) -> Self::OutputPartOne {
        let mut _result: Self::OutputPartOne = 0;

        for _each_elem in _input {
            let _diff = EXPECTED_SUM - _each_elem;
            if _input.contains(&_diff) {
                _result = _each_elem * _diff;
            }
        }

        _result
    }

    fn solve_part_two(_input: &Vec<i64>) -> Self::OutputPartTwo {
        let mut _result: Self::OutputPartTwo = 0;

        for (_i, _each_elem) in _input.into_iter().enumerate() {
            if _i == _input.len() - 1 {
                break;
            }

            let _diff = EXPECTED_SUM - _each_elem;
            let _rest: &Vec<i64> = &_input[_i + 1..].to_vec();

            for _each_rest_elem in _rest {
                let _diff_from_rest = _diff - _each_rest_elem;

                if _rest.contains(&_diff_from_rest) {
                    _result = _each_elem * _each_rest_elem * _diff_from_rest;
                }
            }
        }

        _result
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_1::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            1721,
            979,
            366,
            299,
            675,
            1456
        ];

        // when
        let _res: i64 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 514579);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            1721,
            979,
            366,
            299,
            675,
            1456
        ];

        // when
        let _res: i64 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 241861950);
    }
}