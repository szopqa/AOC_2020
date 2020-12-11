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

    fn can_be_occupied(neighbours: &Vec<(usize, usize, char)>) -> bool {
        neighbours.iter()
            .filter(|(_, _, seat)| *seat == '#')
            .count() == 0
    }

    fn can_be_emptied(neighbours: &Vec<(usize, usize, char)>) -> bool {
        neighbours.iter()
            .filter(|(_, _, seat)| *seat == '#')
            .count() >= 4
    }

    pub fn shuffle_seats(&mut self) {
        let mut seats = self.get_seats_copy();

        let mut changed_state = false;
        for (row_pos, each_row) in self.seats.iter().enumerate() {
            for (pos_in_row, each_seat) in each_row.iter().enumerate() {
                if *each_seat != '.' {
                    println!("X: {}, Y: {}, VAL: {}", &row_pos, &pos_in_row, each_seat);
                    let neighbours = self.get_neighbours(&row_pos, &pos_in_row);
                    if *each_seat != '#' && Self::can_be_occupied(&neighbours) {
                        seats[row_pos][pos_in_row] = '#';
                        changed_state = true;
                    }
                    if *each_seat != 'L' && Self::can_be_emptied(&neighbours) {
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
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_11::*;

    #[test]
    fn test_part_one() {
    }
}