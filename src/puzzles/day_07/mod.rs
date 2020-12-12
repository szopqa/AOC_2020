use std::collections::HashMap;

use crate::puzzles::solution::Solution;

use regex::Regex;

lazy_static! {
    static ref BAGS_QUANTITY_REG: Regex = Regex::new(r"(\d+)\s+(.*)\s+bags*").unwrap();
}

type Bag = String;
type Quantity = u64;
type Bags = HashMap<Bag, HashMap<Bag, Quantity>>;

fn parse_bag<'a>(_input: &'a str) -> (Bag, HashMap<Bag, Quantity>) {
    let mut _bag: Bag= "".to_string();
    let mut _inner: HashMap<Bag, Quantity> = HashMap::new();

    let mut _i: usize = 0;
    loop {
        if _i >= _input.len() {
            break;
        } else if _input[_i..].starts_with("bags contain") {
            _bag = _input[.._i].trim_end().to_string();
            _i = _i + "bags contain ".len(); // shifting to rest of input
            break;
        }
        _i += 1;
    }

    let _rest = &_input[_i.._input.len()-1]; // removing . at the end

    if _rest != "no other bags" {
        let mut _inner_bags_desc: Vec<&str> = _rest.split(',').map(|_s| _s.trim()).collect();
        _inner_bags_desc.into_iter().for_each(|_d| {
            let _matches = BAGS_QUANTITY_REG.captures(_d).unwrap();

            let _quantity  = _matches.get(1).unwrap().as_str();
            let _name = _matches.get(2).unwrap().as_str();

            _inner.insert(_name.to_string(), _quantity.parse::<Quantity>().unwrap());
        });
    }

    (_bag, _inner)
}

fn contains_bag(_bags: &HashMap<Bag, HashMap<Bag, Quantity>>, _outer_bag: &Bag, _bag_to_look_for: &Bag) -> bool {
    _bags
        .get(_outer_bag)
        .unwrap()
        .iter()
        .any(|(_inner_bag, _)| _inner_bag == _bag_to_look_for || contains_bag(_bags, _inner_bag, _bag_to_look_for))
}

fn count_outer_bags(_bags: &HashMap<Bag, HashMap<Bag, Quantity>>, _bag_to_look_for: &Bag) -> usize {
    _bags
        .keys()
        .filter(|_each_bag_color| contains_bag(_bags, _each_bag_color, _bag_to_look_for))
        .count()
}

fn count_nested_bags(_bags: &HashMap<Bag, HashMap<Bag, Quantity>>, _bag_to_look_for: &Bag) -> Quantity {
    _bags
        .get(_bag_to_look_for)
        .unwrap()
        .iter()
        .fold(0, |_acc, (_inner_bag, _inner_bag_quantity)| {
            _acc + *_inner_bag_quantity + *_inner_bag_quantity * count_nested_bags(_bags, _inner_bag)
        })
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = Bag;
    type OutputPartOne = usize;
    type OutputPartTwo = u64;

    fn solve_part_one(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut _bags: Bags = HashMap::new();

        for _each_bag_desc in _input {
            let (_bag, _inner_bags) = parse_bag(_each_bag_desc);
            _bags.insert(_bag, _inner_bags);
        }

        count_outer_bags(&_bags, &"shiny gold".to_string())
    }

    fn solve_part_two(_input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut _bags: Bags = HashMap::new();

        for _each_bag_desc in _input {
            let (_bag, _inner_bags) = parse_bag(_each_bag_desc);
            _bags.insert(_bag, _inner_bags);
        }

        count_nested_bags(&_bags, &"shiny gold".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_07::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: usize = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 4);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input_1 = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].into_iter().map(|_i| String::from(_i)).collect();

        let _input_2 = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags."
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res_1:u64 = Puzzle::solve_part_two(&_input_1);
        let _res_2:u64 = Puzzle::solve_part_two(&_input_2);

        // then
        assert_eq!(_res_1, 32);
        assert_eq!(_res_2, 126);
    }
}