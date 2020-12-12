use crate::puzzles::solution::Solution;

struct Xmas<'a> {
    cipher: &'a Vec<i64>
}

impl <'a> Xmas <'a> {
    pub fn new(cipher: &'a Vec<i64>) -> Self {
        Self {
            cipher
        }
    }

    fn get_possible_sums(preamble: &[i64], res: &mut Vec<i64>) {
        for i in 0..preamble.len() - 1 {
            res.push(preamble[0] + preamble[i + 1])
        }
    }

    fn get_next_possible_values(preamble: &[i64]) -> Vec<i64> {
        let mut out: Vec<i64> = vec![];

        for i in 0..preamble.len() -1 {
            Self::get_possible_sums(&preamble[i..preamble.len()], &mut out);
        }

        out
    }

    pub fn find_corrupted(&self, preamble_len: usize) -> Option<i64> {
        let mut corrupted: Option<i64> = None;
        for i in 0..self.cipher.len() - preamble_len {
            let preamble = &self.cipher[i..i + preamble_len];
            let elem = self.cipher.get(i + preamble_len).unwrap();
            if !Self::get_next_possible_values(&preamble).contains(&elem) {
                corrupted = Some(*elem);
                break;
            }
        }

        corrupted
    }

    pub fn find_range_summing_to_corrupted(&self, corrupted_val: i64) -> i64 {
        let mut contiguous_set = vec![];

        'start_index_loop: for start_i in 0..self.cipher.len() {
            let mut sum: i64 = 0;
            contiguous_set = vec![];

            for end_i in start_i + 1..self.cipher.len() - 1 {
                contiguous_set.push(&self.cipher[end_i]);
                sum += self.cipher[end_i];
                if sum == corrupted_val {
                    break 'start_index_loop;
                } else if sum > corrupted_val {
                    continue 'start_index_loop;
                }
            }
        }

        let min = *contiguous_set.iter().min().unwrap();
        let max = *contiguous_set.iter().max().unwrap();

        min + max
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = i64;
    type OutputPartOne = i64;
    type OutputPartTwo = i64;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let xmas = Xmas::new(input);
        xmas.find_corrupted(25).unwrap()
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let xmas = Xmas::new(input);
        let corrupted_val = xmas.find_corrupted(25).unwrap();
        xmas.find_range_summing_to_corrupted(corrupted_val)
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_09::*;

    #[test]
    fn test_part_one() {
        // given
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];

        let xmas = Xmas::new(&input);

        // when
        let res = xmas.find_corrupted(5).unwrap();

        // then
        assert_eq!(res, 127);
    }

    #[test]
    fn test_part_two() {
        // given
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];

        let xmas = Xmas::new(&input);
        let corrupted_val = xmas.find_corrupted(5).unwrap();

        // when
        let res = xmas.find_range_summing_to_corrupted(corrupted_val);

        // then
        assert_eq!(res, 62);
    }
}