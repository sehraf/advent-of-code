use std::{fmt::Display, ops::Add, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day18";

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailFishNumber {
    Regular(u64),
    Nested(Box<(SnailFishNumber, SnailFishNumber)>),
}

impl From<&str> for SnailFishNumber {
    fn from(s: &str) -> Self {
        if s.chars().nth(0).unwrap() == '[' {
            assert_eq!(s.chars().last().unwrap(), ']');

            // find middle
            let mut stack = 0;
            let mut pos = 0;
            let pos = s
                .chars()
                .find_map(|c| {
                    pos += 1;
                    match c {
                        '[' => {
                            stack += 1;
                            None
                        }
                        ']' => {
                            stack -= 1;
                            None
                        }
                        i if i.is_digit(10) => None,
                        ',' if stack == 1 => Some(pos - 1),
                        ',' if stack != 1 => None,
                        _ => unreachable!("invalid character"),
                    }
                })
                .unwrap();

            let a = SnailFishNumber::from(&s[1..pos]);
            let b = SnailFishNumber::from(&s[pos + 1..s.len() - 1]);
            SnailFishNumber::Nested(Box::new((a, b)))
        } else {
            assert_eq!(s.len(), 1);
            SnailFishNumber::Regular(s.parse().unwrap())
        }
    }
}

impl Add for SnailFishNumber {
    type Output = SnailFishNumber;
    fn add(self, rhs: Self) -> Self::Output {
        SnailFishNumber::Nested(Box::new((self, rhs)))
    }
}

impl SnailFishNumber {
    fn reduce(&mut self) {
        loop {
            // rule 1: explode
            match self.explode(0) {
                (None, None, false) => {}
                (_, _, true) => continue,
                _ => {
                    unreachable!()
                }
            }

            // rule 2: split
            if self.split() {
                continue;
            }

            // the end
            break;
        }
    }

    fn explode(&mut self, depth: u32) -> (Option<u64>, Option<u64>, bool) {
        match self {
            Self::Regular(_) => (None, None, false),
            Self::Nested(bo) if depth >= 4 => {
                if let Self::Regular(a) = bo.0 {
                    if let Self::Regular(b) = bo.1 {
                        (Some(a), Some(b), true)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            Self::Nested(bo) => {
                // left first
                let expl_left = bo.0.explode(depth + 1);
                match expl_left {
                    (None, None, true) => return (None, None, true),
                    (None, None, false) => {}
                    (Some(a), Some(b), true) => {
                        // out left child just exploded!
                        bo.1.sink_increase(b, true);
                        bo.0 = SnailFishNumber::Regular(0);
                        return (Some(a), None, true);
                    }
                    (Some(a), None, true) => {
                        // left child has to propergate a value (further) to the left
                        return (Some(a), None, true);
                    }
                    (None, Some(b), true) => {
                        // left child adds to our right child
                        bo.1.sink_increase(b, true);
                        return (None, None, true);
                    }
                    _ => unreachable!(),
                }

                let expl_right = bo.1.explode(depth + 1);
                match expl_right {
                    (None, None, true) => return (None, None, true),
                    (None, None, false) => {}
                    (Some(a), Some(b), true) => {
                        // out right child just exploded!
                        bo.0.sink_increase(a, false);
                        bo.1 = SnailFishNumber::Regular(0);
                        return (None, Some(b), true);
                    }
                    (Some(a), None, true) => {
                        // right child adds to our left child
                        bo.0.sink_increase(a, false);
                        return (None, None, true);
                    }
                    (None, Some(b), true) => {
                        // right child has to propergate a value (further) to the right
                        return (None, Some(b), true);
                    }
                    _ => unreachable!(),
                }

                // nothing happend
                (None, None, false)
            }
        }
    }

    fn sink_increase(&mut self, val: u64, left: bool) {
        match self {
            Self::Nested(bo) => {
                if left {
                    bo.0.sink_increase(val, left);
                } else {
                    bo.1.sink_increase(val, left);
                }
            }
            Self::Regular(a) => *a += val,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Nested(bo) => bo.0.split() || bo.1.split(),
            Self::Regular(a) if *a >= 10 => {
                let half = *a as f64 / 2.0;
                *self = SnailFishNumber::Nested(Box::new((
                    SnailFishNumber::Regular(half.floor() as u64),
                    SnailFishNumber::Regular(half.ceil() as u64),
                )));
                true
            }
            Self::Regular(_) => false,
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Nested(bo) => bo.0.magnitude() * 3 + bo.1.magnitude() * 2,
            Self::Regular(a) => *a,
        }
    }
}

impl Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nested(bo) => {
                write!(f, "[{},{}]", bo.0, bo.1)
            }
            &Self::Regular(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<SnailFishNumber>,
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
        let lines = input.lines();
        self.input = lines.map(|line| SnailFishNumber::from(line)).collect();
    }

    fn puzzle1(&mut self) -> u64 {
        let mut it = self.input.iter();
        let mut sum = it.next().unwrap().to_owned();
        for sn in it {
            sum = sum + sn.to_owned();
            sum.reduce();
        }
        sum.magnitude()
    }

    fn puzzle2(&mut self) -> u64 {
        let mut max = 0;
        for a in &self.input {
            for b in &self.input {
                if a == b {
                    continue;
                }
                let mut tmp = a.to_owned() + b.to_owned();
                tmp.reduce();
                let tmp = tmp.magnitude();
                if tmp > max {
                    max = tmp;
                }

                let mut tmp = b.to_owned() + a.to_owned();
                tmp.reduce();
                let tmp = tmp.magnitude();
                if tmp > max {
                    max = tmp;
                }
            }
        }
        max
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
        assert_eq!(data.puzzle1(), 4140);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 3993);
    }
}
