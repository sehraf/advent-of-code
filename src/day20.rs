use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day20";
const PAD: usize = 1;

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<bool>,
    image: Vec<Vec<bool>>,
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
        let halfs = input.split_once("\n\n").unwrap();
        self.input = halfs.0.chars().map(|c| c == '#').collect();
        let image: Vec<Vec<bool>> = halfs
            .1
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        // add a "ring" of '.' around the input image
        let max_x = image[0].len();
        let max_y = image.len();
        self.image = vec![vec![false; max_x + PAD * 2]; max_y + PAD * 2];
        for y in 0..max_y {
            for x in 0..max_x {
                self.image[y + PAD][x + PAD] = image[y][x];
            }
        }
    }

    fn puzzle1(&mut self) -> u64 {
        #[cfg(test)]
        {
            // test case has `.` at 0
            self.enhance(false, false);
            self.enhance(false, false);
        }
        #[cfg(not(test))]
        {
            // super mean real case has `#` at 0 and `.` at 511
            self.enhance(false, true);
            self.enhance(true, false);
        }

        self.image
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if *cell { 1 } else { 0 })
                    .sum::<u64>()
            })
            .sum()
    }

    fn puzzle2(&mut self) -> u64 {
        // (0..50).for_each(|i| self.enhance(i % 2 == 0));
        #[cfg(test)]
        {
            // test case has `.` at 0
            (0..50).for_each(|_| self.enhance(false, false));
        }
        #[cfg(not(test))]
        {
            // super mean real case has `#` at 0 and `.` at 511
            (0..50).for_each(|i| self.enhance(i % 2 != 0, i % 2 == 0));
        }

        self.image
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if *cell { 1 } else { 0 })
                    .sum::<u64>()
            })
            .sum()
    }

    fn enhance(&mut self, void: bool, void_new: bool) {
        let max_x = self.image[0].len();
        let max_y = self.image.len();

        let copy = self.image.clone();
        self.image = vec![vec![void_new; max_x + PAD * 2]; max_y + PAD * 2];

        for y in 0..max_y {
            for x in 0..max_x {
                let mut digits = vec![];
                for d in [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (0, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ] {
                    digits.push({
                        let mut input = void;
                        if !(x == 0 && d.0 < 0
                            || y == 0 && d.1 < 0
                            || x == max_x - 1 && d.0 > 0
                            || y == max_y - 1 && d.1 > 0)
                        {
                            let new_x = (x as i64 + d.0) as usize;
                            let new_y = (y as i64 + d.1) as usize;
                            input = copy[new_y][new_x];
                        }
                        // convert
                        if input {
                            '1'
                        } else {
                            '0'
                        }
                    });
                }
                let num = usize::from_str_radix(&String::from_iter(digits), 2).unwrap();
                let lookup = self.input[num];
                self.image[y + PAD][x + PAD] = lookup;
            }
        }
    }

    #[allow(dead_code)]
    fn plot(&self) {
        let max_x = self.image[0].len();
        let max_y = self.image.len();
        for y in 0..max_y {
            for x in 0..max_x {
                if self.image[y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
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
        assert_eq!(data.puzzle1(), 35);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 3351);
    }
}
