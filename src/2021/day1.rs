use std::path::PathBuf;

use crate::AdventOfCode;

#[derive(Debug, Default)]
pub struct Data {
    input: String,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, "day1.txt");

        (self.puzzle1() as u64, self.puzzle2() as u64)
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
        self.input = std::fs::read_to_string(input_file).expect("failed to read file");
    }

    fn puzzle1(&self) -> i32 {
        let mut larger = 0;
        let mut last = None;

        for line in self.input.lines() {
            let num: i32 = line.parse().unwrap();

            last = match last {
                None => Some(num),
                Some(last) => {
                    if last < num {
                        larger += 1;
                    }
                    Some(num)
                }
            }
        }

        larger
    }

    fn puzzle2(&self) -> i32 {
        let mut larger = 0;
        let numbers: Vec<i32> = self.input.lines().map(|str| str.parse().unwrap()).collect();

        for i in 0..numbers.len() - 3 {
            let a: i32 = numbers[i..=i + 2].iter().sum();
            let b: i32 = numbers[i + 1..=i + 3].iter().sum();

            if a < b {
                larger += 1;
            }
        }

        larger
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
        data.load(&base_dir, "day1_test.txt");
        assert_eq!(data.puzzle1(), 7);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, "day1_test.txt");
        assert_eq!(data.puzzle2(), 5);
    }
}
