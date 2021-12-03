use std::path::PathBuf;

use crate::AdventOfCode;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<(Direction, i32)>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) {
        self.load(base_dir, "day2.txt");

        println!("day 2.1: {}", self.puzzle1());
        println!("day 2.2: {}", self.puzzle2());
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: &str) {
        let input_file = base_dir.join(test_input);
        assert!(
            input_file.exists(),
            "input file {} does not exist",
            input_file.to_string_lossy()
        );
        let input = std::fs::read_to_string(input_file).expect("failed to read file");
        self.input = input
            .lines()
            .map(|str| {
                let mut split = str.split_ascii_whitespace();
                let dir = match split.next().expect("failed to split") {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => {
                        unreachable!("unkown direction")
                    }
                };
                let num: i32 = split.next().expect("failed to split").parse().unwrap();
                return (dir, num);
            })
            .collect();
    }

    fn puzzle1(&self) -> i32 {
        let mut x = 0;
        let mut y = 0;

        for action in &self.input {
            match action.0 {
                Direction::Down => y += action.1,
                Direction::Forward => x += action.1,
                Direction::Up => y -= action.1,
            }
        }

        x * y
    }

    fn puzzle2(&self) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for action in &self.input {
            match action.0 {
                Direction::Down => aim += action.1,
                Direction::Forward => {
                    x += action.1;
                    y += aim * action.1;
                }
                Direction::Up => aim -= action.1,
            }
        }

        x * y
    }
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::Data;

    #[test]
    fn puzzle1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, "day2_test.txt");
        assert_eq!(data.puzzle1(), 150);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, "day2_test.txt");
        assert_eq!(data.puzzle2(), 900);
    }
}
