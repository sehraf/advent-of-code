use std::path::PathBuf;

use crate::AdventOfCode;

const DAY: &str = "day10";

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<String>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1();

        // self.load(base_dir, String::from(DAY) + ".txt");
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
        self.input = lines.map(|f| f.to_string()).collect();
    }

    fn puzzle1(&mut self) -> u64 {
        let mut corrupted = vec![];
        self.input.iter().for_each(|line| {
            if let Some(c) = is_corrupted_line(line) {
                corrupted.push(c)
            }
        });
        calc_checker_score(corrupted)
    }

    fn puzzle2(&mut self) -> u64 {
        // filter corrupted lines
        self.input.retain(|line| is_corrupted_line(line).is_none());

        let mut score = vec![];
        self.input.iter().for_each(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                if valid_op_open(&c) {
                    stack.push(c);
                } else if valid_op_close(&c) {
                    let expected = stack.pop();
                    assert!(match_op_code(expected.unwrap(), c));
                }
            }
            score.push(calc_complete_score(stack.into_iter().rev().collect()));
        });

        score.sort();
        score.get(score.len() / 2).unwrap().to_owned()
    }
}

fn is_corrupted_line(line: &str) -> Option<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if valid_op_open(&c) {
            stack.push(c);
        } else if valid_op_close(&c) {
            let expected = stack.pop();
            if expected.is_none() || !match_op_code(expected.unwrap(), c) {
                return Some(c);
            } else {
                // nothing, stack was already popped
            }
        } else {
            assert!(valid_op(&c), "unkown op code");
            unreachable!("illegal instruction");
        }
    }
    None
}

#[allow(dead_code)]
fn valid_op(op: &char) -> bool {
    valid_op_open(op) || valid_op_close(op)
}

fn valid_op_open(op: &char) -> bool {
    match op {
        '{' | '(' | '<' | '[' => true,
        _ => false,
    }
}

fn valid_op_close(op: &char) -> bool {
    match op {
        '}' | ')' | '>' | ']' => true,
        _ => false,
    }
}

fn match_op_code(a: char, b: char) -> bool {
    match a {
        '(' => b == ')',
        '[' => b == ']',
        '{' => b == '}',
        '<' => b == '>',
        _ => unreachable!(),
    }
}

fn calc_checker_score(input: Vec<char>) -> u64 {
    input
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn calc_complete_score(input: Vec<char>) -> u64 {
    input
        .iter()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, score| acc * 5 + score)
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
        assert_eq!(data.puzzle1(), 26397);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 288957);
    }
}
