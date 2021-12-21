use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day3";

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<String>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1() as u64;

        // self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle2() as u64;

        (a, b)
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
        self.input = input.lines().map(|s| s.to_owned()).collect();
    }

    fn puzzle1(&self) -> i32 {
        let mut gamma = 0;
        let mut epsilon = 0;

        let max = self.input.first().unwrap().len();

        for pos in 0..max {
            let mut zero = 0;
            let mut one = 0;

            for entry in &self.input {
                if entry.chars().nth(pos).unwrap() == '1' {
                    one += 1;
                } else {
                    zero += 1;
                }
            }

            gamma <<= 1;
            epsilon <<= 1;
            if one > zero {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        }

        gamma * epsilon
    }

    fn puzzle2(&self) -> i32 {
        let remaining_oxy = self.input.to_owned();
        let remaining_co2 = self.input.to_owned();
        let max = self.input.first().unwrap().len();

        let oxy = filter(max, remaining_oxy, true);
        let co2 = filter(max, remaining_co2, false);

        i32::from_str_radix(&oxy, 2).unwrap() * i32::from_str_radix(&co2, 2).unwrap()
    }
}

fn filter(width: usize, mut input: Vec<String>, equal: bool) -> String {
    for pos in 0..width {
        let mut zero = 0;
        let mut one = 0;

        for entry in &input {
            if entry.chars().nth(pos).unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }

        let common = if one >= zero { '1' } else { '0' };

        input.retain(|value| (value.chars().nth(pos).unwrap() != common) ^ equal);

        if input.len() == 1 {
            break;
        }
    }
    assert_eq!(input.len(), 1);
    input.first().unwrap().to_owned()
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
        assert_eq!(data.puzzle1(), 198);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 230);
    }
}
