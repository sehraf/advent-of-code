use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day7";

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<i32>,
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
        let lines = input.split_terminator(',');
        self.input = lines.map(|s| s.trim_end().parse().unwrap()).collect();
    }

    fn solve(&self, calc_costs: &dyn Fn(&Self, i32) -> i32) -> i32 {
        // start at average position
        let sum = self.input.iter().fold(0, |acc, pos| acc + pos);
        let mut average = sum / self.input.len() as i32;

        // cheap a** Newton's method
        let mut costs = calc_costs(&self, average);
        loop {
            let a = calc_costs(&self, average + 1);
            let b = calc_costs(&self, average - 1);

            if a < costs && a < b {
                average += 1;
            } else if b < costs && b < a {
                average -= 1;
            } else {
                break;
            }
            costs = calc_costs(&self, average);
        }

        costs
    }

    fn puzzle1(&mut self) -> i32 {
        self.solve(&calc_costs_1)
    }

    fn puzzle2(&mut self) -> i32 {
        self.solve(&calc_costs_2)
    }
}

fn sum_thing(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => sum_thing(num - 1) + num,
    }
}

fn calc_costs_1(data: &Data, target_pos: i32) -> i32 {
    if target_pos < 0 {
        return i32::MAX;
    }
    data.input
        .iter()
        .fold(0, |acc, pos| acc + (pos - target_pos).abs())
}

fn calc_costs_2(data: &Data, target_pos: i32) -> i32 {
    if target_pos < 0 {
        return i32::MAX;
    }
    let cost = data.input.iter().fold(0, |acc, pos| {
        acc + sum_thing((pos - target_pos).abs() as u128)
    });
    if cost < i32::MAX as u128 {
        cost as i32
    } else {
        i32::MAX
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
        assert_eq!(data.puzzle1(), 37);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 168);
    }
}
