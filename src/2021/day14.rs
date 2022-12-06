use std::{collections::HashMap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day14";

#[derive(Debug, Default)]
pub struct Data {
    // lanternfish in disguise

    // count occurrences of element tuples
    input: HashMap<(char, char), usize>,
    // count total number of (single) elements
    elements: HashMap<char, usize>,

    // rules
    rules: HashMap<(char, char), char>,
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
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        // insert elem tuples
        self.input.clear();
        first
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .for_each(|f| *self.input.entry((f[0], f[1])).or_default() += 1);

        // insert absolut numbers
        self.elements.clear();
        first
            .chars()
            .for_each(|c| *self.elements.entry(c).or_default() += 1);

        assert!(lines.next().unwrap().is_empty());
        
        // insert rules
        self.rules.clear();
        for rule in lines {
            self.rules.insert(
                (rule.chars().nth(0).unwrap(), rule.chars().nth(1).unwrap()),
                rule.chars().nth(6).unwrap(),
            );
        }
    }

    fn puzzle1(&mut self) -> u64 {
        self.do_chemistry(10);
        (self.elements.values().max().unwrap() - self.elements.values().min().unwrap()) as u64
    }

    fn puzzle2(&mut self) -> u64 {
        self.do_chemistry(40);
        (self.elements.values().max().unwrap() - self.elements.values().min().unwrap()) as u64
    }

    fn do_chemistry(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for ((a, b), n) in self.input.clone() {
                if let Some(&i) = self.rules.get(&(a, b)) {
                    *self.input.entry((a, b)).or_default() -= n;
                    *self.input.entry((a, i)).or_default() += n;
                    *self.input.entry((i, b)).or_default() += n;
                    *self.elements.entry(i).or_default() += n;
                }
            }
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
        assert_eq!(data.puzzle1(), 1588);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 2188189693529);
    }
}
