use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use crate::AdventOfCode;

const DAY: &str = "day11";

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Vec<i32>>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) {
        self.load(base_dir, String::from(DAY) + ".txt");
        println!("{}, puzzle 1: {}", DAY, self.puzzle1());
        
        self.load(base_dir, String::from(DAY) + ".txt");
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
        let lines = input.lines();
        self.input = lines
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
    }

    fn puzzle1(&mut self) -> u64 {
        let mut sum = 0;
        for _ in 0..100 {
            sum += self.step();
        }
        sum
    }

    fn puzzle2(&mut self) -> u64 {
        let mut round = 0;
        loop {
            round += 1; // start at 0 end increase at first

            let flashes = self.step();
            assert!(flashes <= 100);
            if flashes == 100 {
                return round;
            }
        }
    }

    fn step(&mut self) -> u64 {
        let mut visited = HashSet::new();
        let mut flashed = VecDeque::new();
        let mut flashes = 0;

        for y in 0..10 {
            for x in 0..10 {
                self.input[y][x] += 1;
                if self.input[y][x] > 9 {
                    flashed.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = flashed.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }

            flashes += 1;
            // do not reset value here
            visited.insert((x, y));

            for (dir_x, dir_y) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let new_pos = {
                    if x == 0 && dir_x == -1
                        || x == 9 && dir_x == 1
                        || y == 0 && dir_y == -1
                        || y == 9 && dir_y == 1
                    {
                        continue;
                    }
                    (x as i32 + dir_x, y as i32 + dir_y)
                };
                assert!(new_pos.0 >= 0 && new_pos.0 <= 9 && new_pos.1 >= 0 && new_pos.1 <= 9);
                let (new_x, new_y) = (new_pos.0 as usize, new_pos.1 as usize);
                self.input[new_y][new_x] += 1;
                if self.input[new_y][new_x] > 9 {
                    flashed.push_back((new_x, new_y));
                }
            }
        }

        for flashed in visited {
            self.input[flashed.1][flashed.0] = 0;
        }

        flashes
    }

    #[allow(dead_code)]
    fn plot(&self) {
        for y in 0..10 {
            for x in 0..10 {
                print!("{}", self.input[y][x]);
            }
            println!();
        }
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
        assert_eq!(data.puzzle1(), 1656);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 195);
    }
}
