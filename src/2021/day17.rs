use std::{collections::HashSet, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day17";

type Pos2D = (i64, i64);

#[derive(Debug, Default)]
pub struct Data {
    input: (Pos2D, Pos2D),

    part_2: u64,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1();

        self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle2();

        (a, b)
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String) {
        let input_file = base_dir.join(test_input);
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let mut lines = input.lines();
        let mut parts = lines.next().unwrap().split_ascii_whitespace();
        // target area: x=281..311, y=-74..-54
        assert_eq!(parts.next().unwrap(), "target");
        assert_eq!(parts.next().unwrap(), "area:");

        let split_coord = |s: &str| -> (i64, i64) {
            let parts = s.split_once("..").unwrap();

            let x = parts.0[2..].parse().unwrap();
            let y = parts.1.parse().unwrap();

            (x, y)
        };
        let part_x = parts.next().unwrap();
        let (x_1, x_2) = split_coord(&part_x[..part_x.len() - 1]);
        let part_y = parts.next().unwrap();
        let (y_1, y_2) = split_coord(part_y);

        // y_1 is less than y_2!
        self.input = ((x_1, y_1), (x_2, y_2));
    }

    fn puzzle1(&mut self) -> u64 {
        let mut best_height = 0u64;
        let mut num_hits = HashSet::new();

        let min_x = 0; // there is some higher lower bound but it's friday so this is good enough ...
        let max_x = self.input.1 .0; // don't shoot over the target within the first step!
        let min_y = self.input.0 .1; // don't shoot below the target within the first step! (y_1 is less than y_2!)
        let max_y = 500; // guessed and passed everything ...
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Ok(height) = self.simulate_probe((x, y)) {
                    num_hits.insert((x, y));

                    if height > 0 && height as u64 > best_height {
                        best_height = height as u64;
                    }
                }
            }
        }
        self.part_2 = num_hits.len() as u64;

        best_height
    }

    fn puzzle2(&mut self) -> u64 {
        #[cfg(test)]
        self.puzzle1();

        self.part_2
    }

    fn simulate_probe(&self, velocity_in: Pos2D) -> Result<i64, ()> {
        let mut pos = (0, 0);
        let mut velocity = velocity_in;
        let mut max_hight = 0;

        let target_x = self.input.0 .0..=self.input.1 .0;
        let target_y = self.input.0 .1..=self.input.1 .1;

        loop {
            let new_pos = (pos.0 + velocity.0, pos.1 + velocity.1);

            if new_pos.1 > max_hight {
                max_hight = new_pos.1;
            }

            // are we there yet?
            if target_x.contains(&new_pos.0) && target_y.contains(&new_pos.1) {
                return Ok(max_hight);
            }

            // too far?
            // y_1 is less than y_2!
            if new_pos.0 > self.input.1 .0 || new_pos.1 < self.input.0 .1 {
                return Err(());
            }

            if velocity.0 > 0 {
                velocity.0 -= 1;
            } else if velocity.0 < 0 {
                velocity.0 += 1;
            }
            velocity.1 -= 1;

            pos = new_pos;
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
        assert_eq!(data.puzzle1(), 45);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 112);
    }
}
