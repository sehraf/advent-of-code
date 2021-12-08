use std::{collections::HashMap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day8";

const KNOWN_DIGIT_LENGHTS: &[(usize, i32)] = &[(2, 1), (3, 7), (4, 4), (7, 8)];

type Digit = String;

#[derive(Debug)]
struct Segment {
    /// Our unique signal patterns
    signals: [Digit; 10],
    /// Out four digit output value
    output: [Digit; 4],

    /// Known mapping (seen char -> correct char)
    mapping: HashMap<char, char>,
}

fn digit_to_int(digit: &str) -> Option<i32> {
    // sort so we can match against known values
    let mut sorted: Vec<char> = digit.chars().collect::<Vec<char>>();
    sorted.sort_by(|a, b| a.cmp(b));
    let sorted = String::from_iter(sorted);

    // lookup values
    let num = match sorted.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        any => {
            println!("{} is no known number", any);
            return None;
        }
    };
    Some(num)
}

impl Segment {
    fn try_solve(&self) -> Option<i32> {
        let mut ret = 0;
        for output in &self.output {
            // is the mapping complete?
            if self.mapping.len() != 7 {
                return None;
            }
            let solution = String::from_iter(output.chars().map(|c| self.mapping.get(&c).unwrap()));

            // lookup number
            if let Some(num) = digit_to_int(&solution) {
                ret *= 10;
                ret += num;
            } else {
                unreachable!("found a solution that is not a solution !?");
            }
        }
        Some(ret)
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Segment>,
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
        let lines = input.lines();
        self.input = lines
            .map(|s| {
                let mut halfs = s.split_terminator('|');

                // unique signals
                let signals = halfs
                    .next()
                    .unwrap()
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
                    .try_into()
                    .unwrap_or_else(|v: Vec<Digit>| {
                        panic!("Expected a Vec of length {} but it was {}", 10, v.len())
                    });
                // ouput
                let output = halfs
                    .next()
                    .unwrap()
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
                    .try_into()
                    .unwrap_or_else(|v: Vec<Digit>| {
                        panic!("Expected a Vec of length {} but it was {}", 4, v.len())
                    });

                Segment {
                    signals,
                    output,
                    mapping: HashMap::new(),
                }
            })
            .collect();
    }

    fn solve(&mut self) {
        // Note from future me:
        // After learning that HashSets has `is_subset`, `union` and similar functions, i realise that this could have been solved way cleaner.
        // But hey, it does work!

        // assumption: input is designed to always work naive ...
        for segment in &mut self.input {
            // find 1
            let one = segment.signals.iter().find(|s| s.len() == 2).unwrap();
            // find 7
            let seven = segment.signals.iter().find(|s| s.len() == 3).unwrap();
            // find 4
            let four = segment.signals.iter().find(|s| s.len() == 4).unwrap();
            // find 8
            let eight = segment.signals.iter().find(|s| s.len() == 7).unwrap();

            // what we know:
            //  - there is only one difference between 1 and 7
            // --> we learn position a
            // --> we learn candidates for position c and f
            // --> we learn candidates for position b and d
            let mapped_char_a = seven
                .chars()
                .find(|c| !one.contains(&c.to_string()))
                .unwrap();
            let candidates_c_f = one;
            let candidates_b_d =
                String::from_iter(four.chars().filter(|c| !one.contains(&c.to_string())));

            // what we know:
            //  - 4 is part of 9 and 8
            //  - we can distinguish 9
            //  - we know position a
            //  - there is only difference between 4 and 9 (excluding pos. a)
            // --> we learn postion g
            let nine = segment
                .signals
                .iter()
                .find(|s| {
                    if s.len() != 6 {
                        return false;
                    }
                    let mut good = true;
                    four.chars().for_each(|c| {
                        if !s.contains(&c.to_string()) {
                            good = false;
                        }
                    });
                    good
                })
                .unwrap();
            let mapped_char_g = nine
                .chars()
                .find(|c| c != &mapped_char_a && !four.contains(&c.to_string()))
                .unwrap();

            // what we know:
            //  - there is one difference between 8 and 9
            // --> we learn position e
            let mapped_char_e = eight
                .chars()
                .find(|c| !nine.contains(&c.to_string()))
                .unwrap();

            // what we know:
            //  - we can identify 0 (all positions known or known candidates)
            //  - 0 helps to distinguish between b and d candidates
            // --> we learn position b and d
            let zero = segment
                .signals
                .iter()
                .find(|s| {
                    if s.len() != 6 || s == &nine {
                        return false;
                    }
                    let mut good = true;
                    candidates_c_f.chars().for_each(|c| {
                        if !s.contains(&c.to_string()) {
                            good = false;
                        }
                    });
                    good
                })
                .unwrap();
            let mapped_char_b = zero
                .chars()
                .find(|c| candidates_b_d.contains(&c.to_string()))
                .unwrap();
            let mapped_char_d = candidates_b_d
                .chars()
                .find(|c| c != &mapped_char_b)
                .unwrap();

            // what we know:
            //  - we can identify 6 (two of three six char. wide numbers are known)
            //  - 6 helps to distinguish between c and f candidates
            // --> we learn position f and c
            let six = segment
                .signals
                .iter()
                .find(|s| s.len() == 6 && s != &nine && s != &zero)
                .unwrap();
            let mapped_char_f = six
                .chars()
                .find(|c| candidates_c_f.contains(&c.to_string()))
                .unwrap();
            let mapped_char_c = candidates_c_f
                .chars()
                .find(|c| c != &mapped_char_f)
                .unwrap();

            segment.mapping.insert(mapped_char_a, 'a');
            segment.mapping.insert(mapped_char_b, 'b');
            segment.mapping.insert(mapped_char_c, 'c');
            segment.mapping.insert(mapped_char_d, 'd');
            segment.mapping.insert(mapped_char_e, 'e');
            segment.mapping.insert(mapped_char_f, 'f');
            segment.mapping.insert(mapped_char_g, 'g');
        }
    }

    fn puzzle1(&mut self) -> i32 {
        let mut solution = 0;
        self.input.iter().for_each(|segment| {
            segment.output.iter().for_each(|s| {
                for known_len in KNOWN_DIGIT_LENGHTS {
                    if s.len() == known_len.0 {
                        solution += 1;
                    }
                }
            })
        });
        solution
    }

    fn puzzle2(&mut self) -> i32 {
        self.solve();
        self.input
            .iter()
            .fold(0, |acc, segment| acc + segment.try_solve().unwrap())
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
        assert_eq!(data.puzzle1(), 26);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 61229);
    }
}
