use std::{collections::HashMap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day6";

#[derive(Debug)]
struct FishSchool {
    count: u64,
    timer: i32,
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<FishSchool>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1(80) as u64;

        // self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle1(256 - 80) as u64; // this is essential!! self.input is not reset between calls!

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

        // prepare input
        let lines = input.split_terminator(',');
        let input: Vec<i32> = lines.map(|s| s.trim_end().parse().unwrap()).collect();

        let mut school = HashMap::new();
        for entry in input {
            school
                .entry(entry)
                .or_insert(FishSchool {
                    count: 0,
                    timer: entry.to_owned(),
                })
                .count += 1;
        }
        self.input = school.into_values().collect();
    }

    fn tick_schools(&mut self) {
        // tick(le) fishes
        let mut newborn = 0;
        self.input.iter_mut().for_each(|school| {
            if school.timer == 0 {
                newborn += school.count;
                school.timer = 6;
            } else {
                school.timer -= 1;
            }
        });

        // // condense groups (optional, to keep list small)
        // let removed = self
        //     .input
        //     .iter()
        //     .filter(|school| school.timer == 6)
        //     .fold(0, |acc, school| acc + school.count);
        // self.input.retain(|school| school.timer != 6);
        // self.input.push(FishSchool {
        //     count: removed,
        //     timer: 6,
        // });

        // reproduce
        if newborn > 0 {
            self.input.push(FishSchool {
                count: newborn,
                timer: 8,
            })
        }
    }

    fn puzzle1(&mut self, days: i32) -> u64 {
        for _ in 0..days {
            self.tick_schools();
        }

        self.input.iter().fold(0, |acc, school| acc + school.count)
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
        assert_eq!(data.puzzle1(80), 5934);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle1(256), 26984457539);
    }
}
