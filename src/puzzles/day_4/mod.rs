use crate::puzzles::solution::Solution;

use regex::Regex;

pub struct Puzzle {}


lazy_static! {
    // static ref REG: Regex = Regex::new(r"byr:(\d+)|iyr:(\d+)|eyr:(\d+)|hgt:(\w+)|hcl:(#\w+)|ecl:(\w+)|pid:(\w+)|cid:(\d+)").unwrap();
    
    static ref BYR_REG: Regex = Regex::new(r"byr:(\d+)").unwrap();
    static ref IYR_REG: Regex = Regex::new(r"iyr:(\d+)").unwrap();
    static ref EYR_REG: Regex = Regex::new(r"eyr:(\d+)").unwrap();
    static ref HGT_REG: Regex = Regex::new(r"hgt:(\w+)").unwrap();
    static ref HCL_REG: Regex = Regex::new(r"hcl:(\W?\w+)").unwrap();
    static ref ECL_REG: Regex = Regex::new(r"ecl:(\W?\w+)").unwrap();
    static ref PID_REG: Regex = Regex::new(r"pid:(\W?\w+)").unwrap();
    static ref CID_REG: Regex = Regex::new(r"cid:(\d+)").unwrap();
}

#[derive(Debug)]
struct PassValidator <'a> {
    _byr: Option<&'a str>,
    _iyr: Option<&'a str>,
    _eyr: Option<&'a str>,
    _hgt: Option<&'a str>,
    _hcl: Option<&'a str>,
    _ecl: Option<&'a str>,
    _pid: Option<&'a str>,
    _cid: Option<&'a str>
}

impl <'a> PassValidator <'a> {
    pub fn new(_pass_line: &'a str) -> Self {
        Self {
            _byr: match BYR_REG.captures(_pass_line) {
                Some(_byr) => Some(_byr.get(1).unwrap().as_str()),
                None => None
            },
            _iyr: match IYR_REG.captures(_pass_line) {
                Some(_iyr) => Some(_iyr.get(1).unwrap().as_str()),
                None => None
            },
            _eyr: match EYR_REG.captures(_pass_line) {
                Some(_eyr) => Some(_eyr.get(1).unwrap().as_str()),
                None => None
            },
            _hgt: match HGT_REG.captures(_pass_line) {
                Some(_hgt) => Some(_hgt.get(1).unwrap().as_str()),
                None => None
            },
            _hcl: match HCL_REG.captures(_pass_line) {
                Some(_hcl) => Some(_hcl.get(1).unwrap().as_str()),
                None => None
            },
            _ecl: match ECL_REG.captures(_pass_line) {
                Some(_ecl) => Some(_ecl.get(1).unwrap().as_str()),
                None => None
            },
            _pid: match PID_REG.captures(_pass_line) {
                Some(_pid) => Some(_pid.get(1).unwrap().as_str()),
                None => None
            },
            _cid: match CID_REG.captures(_pass_line) {
                Some(_cid) => Some(_cid.get(1).unwrap().as_str()),
                None => None
            },
        }
    }

    pub fn is_valid_part_one(&self) -> bool {
        self._byr.is_some() && self._iyr.is_some() && self._eyr.is_some() && self._hgt.is_some()
            && self._hcl.is_some() && self._ecl.is_some() && self._pid.is_some()
    }
}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = usize;
    type OutputPartTwo = usize;

    fn normalize_input(_input: Vec<Self::PuzzleInput>) -> Vec<Self::PuzzleInput> {
        let mut _normalized_input = vec![];

        let mut _line_normalized: String = "".to_string();
        for _line in _input {
            if _line != "" {
                _line_normalized.push_str(&_line);
                _line_normalized.push_str(" ");
            } else {
                _normalized_input.push(_line_normalized);
                _line_normalized = "".to_string();
            }
        }

        _normalized_input.push(_line_normalized); // last line

        _normalized_input
    }    

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut _result: Self::OutputPartOne = 0;
        
        _input
            .iter()
            .map(|_passport| PassValidator::new(_passport))
            .filter(|_v| _v. is_valid_part_one())
            .collect::<Vec<PassValidator>>()
            .len()

    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut _result: Self::OutputPartTwo = 0;

        _result
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_4::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 2);
    }

    #[test]
    fn test_part_two() {
    }
}