use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day4";

#[derive(Debug)]
struct Board {
    data: [i32; 25],
    marked: [bool; 25],
}

impl Board {
    pub fn new(input: Vec<i32>) -> Self {
        assert_eq!(input.len(), 25, "wrong input lenght");

        Board {
            data: input.try_into().unwrap(),
            marked: [false; 25],
        }
    }

    pub fn check_number(&mut self, number: &i32) -> bool {
        for i in 0..self.data.len() {
            if &self.data[i] == number {
                self.marked[i] = true;
            }
        }

        self.check()
    }

    pub fn check(&self) -> bool {
        for i in 0..5 {
            // rows
            if self.marked[0 + i * 5]
                && self.marked[1 + i * 5]
                && self.marked[2 + i * 5]
                && self.marked[3 + i * 5]
                && self.marked[4 + i * 5]
            {
                return true;
            }

            // columns
            if self.marked[0 + i]
                && self.marked[5 + i]
                && self.marked[10 + i]
                && self.marked[15 + i]
                && self.marked[20 + i]
            {
                return true;
            }
        }

        false
    }

    fn score_unmarked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.marked.len() {
            if !self.marked[i] {
                sum += self.data[i];
            }
        }
        sum
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<i32>,
    boards: Vec<Board>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) {
        self.load(base_dir, String::from(DAY) + ".txt");

        println!("{}, puzzle 1: {}", DAY, self.puzzle1());
        println!("{}, puzzle 2: {}", DAY, self.puzzle2());
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String) {
        let input_file = base_dir.join(test_input);
        assert!(
            input_file.exists(),
            "input file {} does not exist",
            input_file.to_string_lossy()
        );
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let mut lines = input.lines();
        self.input = lines
            .next()
            .unwrap()
            .split_terminator(',')
            .map(|s| s.parse().expect("failed to parse str"))
            .collect();
        assert!(lines.next().unwrap() == String::new());

        // fill boards
        while let Some(next_line) = lines.next() {
            let mut board = next_line.to_owned();
            for _ in 0..4 {
                board += " ";
                board += lines.next().unwrap();
            }
            let board = Board::new(
                board
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
            self.boards.push(board);

            if lines.next().is_none() {
                break;
            }
        }
    }

    fn puzzle1(&mut self) -> i32 {
        for number in &self.input {
            for board in &mut self.boards {
                if board.check_number(number) {
                    return board.score_unmarked() * number;
                }
            }
        }
        -1
    }

    fn puzzle2(&mut self) -> i32 {
        for number in &self.input {
            self.boards.iter_mut().for_each(|board| {
                board.check_number(number);
            });

            // Assume that there won't be multiple winners at the same time
            if self.boards.len() == 1 {
                // are we there yet?
                let board = self.boards.first().unwrap();
                if board.check() {
                    return board.score_unmarked() * number;
                }
            } else {
                self.boards.retain(|board| !board.check());
            }
        }
        -1
    }
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::{Data, DAY};

    #[test]
    fn puzzle1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle1(), 4512);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 1924);
    }
}
