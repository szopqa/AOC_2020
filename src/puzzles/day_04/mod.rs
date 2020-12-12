use crate::puzzles::solution::Solution;

use regex::Regex;

pub struct Puzzle {}


lazy_static! {
    static ref BYR_REG: Regex = Regex::new(r"byr:(\d+)").unwrap();
    static ref IYR_REG: Regex = Regex::new(r"iyr:(\d+)").unwrap();
    static ref EYR_REG: Regex = Regex::new(r"eyr:(\d+)").unwrap();
    static ref HGT_REG: Regex = Regex::new(r"hgt:(\w+)").unwrap();
    static ref HCL_REG: Regex = Regex::new(r"hcl:(\W?\w+)").unwrap();
    static ref ECL_REG: Regex = Regex::new(r"ecl:(\W?\w+)").unwrap();
    static ref PID_REG: Regex = Regex::new(r"pid:(\W?\w+)").unwrap();
    static ref CID_REG: Regex = Regex::new(r"cid:(\d+)").unwrap();

    static ref VALID_HEX_REG: Regex = Regex::new(r"[^0-9a-f]").unwrap();
    static ref VALID_PID_REG: Regex = Regex::new(r"[^0-9]").unwrap();
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

    pub fn contains_mandatory_fields(&self) -> bool {
        self._byr.is_some() && self._iyr.is_some() && self._eyr.is_some() && self._hgt.is_some()
            && self._hcl.is_some() && self._ecl.is_some() && self._pid.is_some()
    }

    pub fn contains_valid_values(&self) -> bool {
        let _byr_value = self._byr.unwrap().parse::<u32>().unwrap();
        let _iyr_value = self._iyr.unwrap().parse::<u32>().unwrap();
        let _eyr_value = self._eyr.unwrap().parse::<u32>().unwrap();

        let _hgt_value = self._hgt.unwrap();
        let _hcl_value = self._hcl.unwrap();
        let _ecl_value = self._ecl.unwrap();
        let _pid_value = self._pid.unwrap();

        let _has_valid_byr = _byr_value >= 1920 && _byr_value <= 2002;
        let _has_valid_iyr = _iyr_value >= 2010 && _iyr_value <= 2020;
        let _has_valid_eyr = _eyr_value >= 2020 && _eyr_value <= 2030;

        let _has_valid_hgt: bool = match &_hgt_value[_hgt_value.len()-2..] {
            "cm" => {
                let _val = _hgt_value[.._hgt_value.len()-2].parse::<u8>().unwrap();
                _val >= 150 && _val <= 193
            },
            "in" => {
                let _val = _hgt_value[.._hgt_value.len()-2].parse::<u8>().unwrap();
                _val >= 59 && _val <= 76
            },
            _ => false
        };

        let _has_valid_hcl: bool = match &_hcl_value[..1] {
            "#" => {
                let _length_matches = &_hcl_value[1..].len() == &(6 as usize);
                if !_length_matches {
                    return false
                }

                VALID_HEX_REG.captures(&_hcl_value[1..]).is_none()
            },
            _ => false
        };

        let _has_valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&_ecl_value);

        let _has_valid_pid = _pid_value.len() == 9 as usize && VALID_PID_REG.captures(&_pid_value).is_none();

        _has_valid_byr && _has_valid_ecl && _has_valid_eyr && _has_valid_hcl
            && _has_valid_hcl && _has_valid_hgt && _has_valid_iyr && _has_valid_pid
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
        _input
            .iter()
            .map(|_passport| PassValidator::new(_passport))
            .filter(|_v| _v. contains_mandatory_fields())
            .collect::<Vec<PassValidator>>()
            .len()

    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        _input
            .iter()
            .map(|_passport| PassValidator::new(_passport))
            .filter(|_v| _v. contains_mandatory_fields())
            .filter(|_v| _v. contains_valid_values())
            .collect::<Vec<PassValidator>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_04::*;

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
        // given
        let _input = vec![
            // invalid
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
            //valid
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 4);
    }
}