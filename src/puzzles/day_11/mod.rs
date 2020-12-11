use crate::puzzles::solution::Solution;

struct Ferry {
    seats: Vec<Vec<char>>,
    pub seats_state_stable: bool
}

impl Ferry {
    pub fn new(seats: Vec<Vec<char>>) -> Self {
        Self {
            seats,
            seats_state_stable: false
        }
    }

    fn get_seats_copy(&self) -> Vec<Vec<char>> {
        self.seats.clone()
    }

    fn get_neighbours(&self, x_pos: &usize, y_pos: &usize) -> Vec<(usize, usize, char)> {
        let mut neighbours: Vec<(usize, usize, char)> = vec![];

        let x_range = if *x_pos > 0 { x_pos - 1..x_pos + 2 } else { 0..2 };

        for x in x_range {
            match self.seats.get(x) {
                Some(row) => {
                    let y_range = if *y_pos > 0 { y_pos - 1..y_pos + 2 } else { 0..2 };

                    for y in y_range {
                        match row.get(y) {
                            Some(seat) => {
                                if *y_pos != y || *x_pos != x {
                                    neighbours.push((*x_pos, *y_pos, *seat))
                                }
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            };
        }

        neighbours
    }

    fn get_far_neighbours(&self, x_pos: &usize, y_pos: &usize) -> Vec<(usize, usize, char)> {
        let mut neighbours: Vec<(usize, usize, char)> = vec![];

        let x_incomes:Vec<isize> = if *x_pos > 0 { vec![-1, 0, 1] } else { vec![0, 1] };
        let y_incomes:Vec<isize> = if *y_pos > 0 { vec![-1, 0, 1] } else { vec![0, 1] };

        for a in &x_incomes {
            for b in &y_incomes {
                let mut x = (*x_pos as isize + a) as usize;
                let mut y = (* y_pos as isize + b) as usize;

                loop {
                    if x > self.seats.len()-1 || y > self.seats.get(0).unwrap().len()-1 {
                        break;
                    }

                    let n =  &self.seats[x][y];
                    if *n != '.' {
                        if *x_pos != x || *y_pos != y {
                            neighbours.push((x, y, *n));
                        }
                        break;
                    }

                    x = (x as isize + a) as usize;
                    y = (y as isize + b) as usize;
                }
            }
        }

        neighbours
    }

    fn can_be_occupied(neighbours: &Vec<(usize, usize, char)>) -> bool {
        neighbours.iter()
            .filter(|(_, _, seat)| *seat == '#')
            .count() == 0
    }

    fn can_be_emptied(neighbours: &Vec<(usize, usize, char)>, min_num: usize) -> bool {
        neighbours.iter()
            .filter(|(_, _, seat)| *seat == '#')
            .count() >= min_num
    }

    pub fn shuffle_seats(&mut self) {
        let mut seats = self.get_seats_copy();

        let mut changed_state = false;
        for (row_pos, each_row) in self.seats.iter().enumerate() {
            for (pos_in_row, each_seat) in each_row.iter().enumerate() {
                if *each_seat != '.' {
                    let neighbours = self.get_neighbours(&row_pos, &pos_in_row);
                    if *each_seat != '#' && Self::can_be_occupied(&neighbours) {
                        seats[row_pos][pos_in_row] = '#';
                        changed_state = true;
                    }
                    if *each_seat != 'L' && Self::can_be_emptied(&neighbours, 4) {
                        seats[row_pos][pos_in_row] = 'L';
                        changed_state = true;
                    }
                }
            }
        }

        if !changed_state {
            self.seats_state_stable = true
        }

        self.seats = seats;
    }

    pub fn shuffle_far_seats(&mut self) {
        let mut seats = self.get_seats_copy();

        let mut changed_state = false;
        for (row_pos, each_row) in self.seats.iter().enumerate() {
            for (pos_in_row, each_seat) in each_row.iter().enumerate() {
                if *each_seat != '.' {
                    let neighbours = self.get_far_neighbours(&row_pos, &pos_in_row);
                    if *each_seat != '#' && Self::can_be_occupied(&neighbours) {
                        seats[row_pos][pos_in_row] = '#';
                        changed_state = true;
                    }
                    if *each_seat != 'L' && Self::can_be_emptied(&neighbours, 5) {
                        seats[row_pos][pos_in_row] = 'L';
                        changed_state = true;
                    }
                }
            }
        }

        if !changed_state {
            self.seats_state_stable = true
        }

        self.seats = seats;
    }

    pub fn get_occupied(&self) -> u64 {
        let mut ans = 0;
        for r in &self.seats {
            for s in r {
                if *s == '#' {
                    ans += 1;
                }
            }
        }
        ans
    }
}

pub struct Puzzle {}

impl Solution for Puzzle {
    type PuzzleInput = String;
    type OutputPartOne = u64;
    type OutputPartTwo = u64;

    fn solve_part_one(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartOne {
        let mut seats: Vec<Vec<char>> = vec![vec![]; input.len()];
        for (i, each_row) in input.iter().enumerate() {
            seats[i] = each_row.chars().map(|c| c).collect();
        }

        let mut ferry = Ferry::new(seats);

        while !ferry.seats_state_stable {
            ferry.shuffle_seats();
        }

        ferry.get_occupied()
    }

    fn solve_part_two(input: &Vec<Self::PuzzleInput>) -> Self::OutputPartTwo {
        let mut seats: Vec<Vec<char>> = vec![vec![]; input.len()];
        for (i, each_row) in input.iter().enumerate() {
            seats[i] = each_row.chars().map(|c| c).collect();
        }

        let mut ferry = Ferry::new(seats);

        while !ferry.seats_state_stable {
            ferry.shuffle_far_seats();
        }

        ferry.get_occupied()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_11::*;

    #[test]
    fn test_part_one() {
        // given
        let _input = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_one(&_input);

        // then
        assert_eq!(_res, 37);
    }

    #[test]
    fn test_part_two() {
        // given
        let _input = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ].into_iter().map(|_i| String::from(_i)).collect();

        // when
        let _res: u64 = Puzzle::solve_part_two(&_input);

        // then
        assert_eq!(_res, 26);
    }
}