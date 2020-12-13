use crate::puzzles::solution::Solution;

#[derive(Debug, Copy, Clone)]
struct Bus {
    pub id: u64,
    relative_timestamp_offset: usize
}

impl Bus {
    pub fn new(id: String, timestamp_offset: usize) -> Self {
        Self {
            id: id.parse::<u64>().unwrap(),
            relative_timestamp_offset: timestamp_offset
        }
    }

    fn get_earliest(&self, timestamp: u64) -> u64 {
        if timestamp % self.id == 0 {
            return timestamp
        };

        let start = timestamp / self.id; // float to unsigned int

        start * self.id + self.id
    }

    fn get_timestamp_for_iteration(&self, iteration: u64) -> u64 {
        iteration * self.id as u64
    }

    fn is_ok_relative_to_first(&self, prev_timestamp: u64, timestamp: u64) -> bool {
        prev_timestamp + self.relative_timestamp_offset as u64 == timestamp
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let operating_buses: Vec<Bus> = input
            .get(1).unwrap()
            .split(',')
            .filter(|i| *i != "x")
            .map(|i| Bus::new(i.to_string(), 0))
            .collect();

        let earliest_timestamp = input.get(0).unwrap().parse::<u64>().unwrap();

        let mut min = u64::MAX;
        let mut ans = 0;

        for b in &operating_buses {
            let closest_timestamp = b.get_earliest(earliest_timestamp);
            if closest_timestamp < min {
                min = closest_timestamp;
                ans = (min - earliest_timestamp) * b.id;
            }
        }

        ans
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let operating_buses: Vec<Bus> = input
            .get(1).unwrap()
            .split(',')
            .enumerate()
            .map(|(i, b)| {
                if b != "x" {
                    return Some(Bus::new(b.to_string(), i))
                }
                None
            })
            .filter(|b| b.is_some())
            .map(|b| b.unwrap())
            .collect();
        
        let mut iter: u64 = 1;
        loop {
            iter += 1;

            let mut busses_checked = 0;
            let mut first_bus_timestamp: u64 = 0;
            let mut should_skip = false;

            for b in &operating_buses {
                busses_checked += 1;
                if should_skip == true {
                    should_skip = false;
                    break;
                }

                if busses_checked == 1 {
                    let timestamp = b.get_timestamp_for_iteration(iter);
                    first_bus_timestamp = timestamp;
                } else {
                    let timestamp = b.get_earliest(first_bus_timestamp);
                    if !b.is_ok_relative_to_first(first_bus_timestamp, timestamp) {
                        should_skip = true;
                        break;
                    }
                }
            }

            if !should_skip {
                return first_bus_timestamp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_13::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "939",
            "7,13,x,x,59,x,31,19"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 295);
    }

    #[test]
    fn test_part_two() {
        // given
        let input1 = vec![
            "_",
            "7,13,x,x,59,x,31,19"
        ].into_iter().map(|_i| String::from(_i)).collect();

        let input2 = vec![
            "_",
            "17,x,13,19"
        ].into_iter().map(|_i| String::from(_i)).collect();

        let input3 = vec![
            "_",
            "67,7,59,61"
        ].into_iter().map(|_i| String::from(_i)).collect();

        let input4 = vec![
            "_",
            "67,x,7,59,61"
        ].into_iter().map(|_i| String::from(_i)).collect();

        let input5 = vec![
            "_",
            "67,7,x,59,61"
        ].into_iter().map(|_i| String::from(_i)).collect();

        let input6 = vec![
            "_",
            "1789,37,47,1889"
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let res1 = Puzzle::solve_part_two(&input1);
        let res2 = Puzzle::solve_part_two(&input2);
        let res3 = Puzzle::solve_part_two(&input3);
        let res4 = Puzzle::solve_part_two(&input4);
        let res5 = Puzzle::solve_part_two(&input5);
        let res6 = Puzzle::solve_part_two(&input6);

        // then
        assert_eq!(res1, 1068781);
        assert_eq!(res2, 3417);
        assert_eq!(res3, 754018);
        assert_eq!(res4, 779210);
        assert_eq!(res5, 1261476);
        assert_eq!(res6, 1202161486);
    }
}