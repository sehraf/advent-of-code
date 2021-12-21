use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day13";

#[derive(Debug, Default)]
struct Fold {
    pos: usize,
    is_x: bool,
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Vec<bool>>,
    folds: Vec<Fold>,
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
        assert!(
            input_file.exists(),
            "input file {} does not exist",
            input_file.to_string_lossy()
        );
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let lines = input.lines();
        let mut dots = vec![];
        let mut max_x = 0;
        let mut max_y = 0;
        self.folds.clear();
        self.input.clear();
        for line in lines {
            // check empty line
            if line.is_empty() {
                continue;
            }

            // check for folds
            if line.contains("fold") {
                let mut parts = line.split_ascii_whitespace();
                assert_eq!(parts.next().unwrap(), "fold");
                assert_eq!(parts.next().unwrap(), "along");
                let mut fold = parts.next().unwrap().split('=');
                self.folds.push(Fold {
                    is_x: fold.next().unwrap() == "x",
                    pos: fold.next().unwrap().parse().unwrap(),
                });
                continue;
            }

            // handle dots
            let mut coords = line.split(',');
            let x: usize = coords.next().unwrap().parse().unwrap();
            let y: usize = coords.next().unwrap().parse().unwrap();
            dots.push((x, y));
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        for y in 0..=max_y {
            let mut v = vec![];
            for x in 0..=max_x {
                v.push(if dots.contains(&(x, y)) { true } else { false });
            }
            self.input.push(v)
        }
    }

    fn puzzle1(&mut self) -> u64 {
        self.fold(false);
        self.input.iter().flatten().filter(|&&b| b).count() as u64
    }

    fn puzzle2(&mut self) -> u64 {
        self.fold(true);
        self.plot();

        0
    }

    fn fold(&mut self, task_two: bool) {
        for fold in &self.folds {
            if fold.is_x {
                // walk columns that get folded
                let range_start = fold.pos + 1;
                let range_end = self.input[0].len();

                for row_cnt in 0..self.input.len() {
                    for col_cnt in range_start..range_end {
                        // calculate impacted columns
                        let src_col = col_cnt;
                        let dst_col = fold.pos - (col_cnt - fold.pos);

                        // apply fold
                        self.input[row_cnt][dst_col] |= self.input[row_cnt][src_col];
                    }

                    // strip folded columns
                    for _ in 0..=fold.pos {
                        self.input[row_cnt].pop();
                    }
                }
            } else {
                // walk rows that get folded
                let range_start = fold.pos + 1;
                let range_end = self.input.len();
                for row_cnt in range_start..range_end {
                    // calculate impacted rows
                    let src_row = row_cnt;
                    let dst_row = fold.pos - (row_cnt - fold.pos);

                    // apply fold
                    for col_cnt in 0..self.input[src_row].len() {
                        self.input[dst_row][col_cnt] |= self.input[src_row][col_cnt];
                    }
                }

                // strip folded rows
                for _ in 0..=fold.pos {
                    self.input.pop();
                }
            }

            if !task_two {
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn plot(&self) {
        for y in &self.input {
            for x in y {
                if *x {
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
        assert_eq!(data.puzzle1(), 17);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 0);
    }
}
