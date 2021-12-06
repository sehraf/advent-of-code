use std::{collections::HashMap, iter::Map, path::PathBuf, vec};

use crate::AdventOfCode;

const DAY: &str = "day5";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Vents {
    a: Coords,
    b: Coords,
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Vents>,
    map: HashMap<Coords, i32>,
    // max_x: i32,
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
                // start
                let mut sides = s.split_terminator("->");
                let mut tmp = sides.next().unwrap().split_terminator(',');
                let a = Coords {
                    x: tmp.next().unwrap().trim().parse().unwrap(),
                    y: tmp.next().unwrap().trim().parse().unwrap(),
                };

                let mut tmp = sides.next().unwrap().split_terminator(',');
                let b = Coords {
                    x: tmp.next().unwrap().trim().parse().unwrap(),
                    y: tmp.next().unwrap().trim().parse().unwrap(),
                };

                Vents { a, b }
            })
            .collect();
    }

    fn build_map(&mut self, include_diag: bool) {
        self.map.clear();
        for vent in &self.input {
            // check validity
            let straight = vent.a.x == vent.b.x || vent.a.y == vent.b.y;
            let diag = (vent.a.x - vent.b.x).abs() == (vent.a.y - vent.b.y).abs();

            if (!include_diag && !straight) || (include_diag && (!straight && !diag)) {
                continue;
            }

            let mut pos = vent.a.clone();
            // cheap a** do while, don't judge me
            while {
                let entry = self.map.entry(pos).or_insert(0);
                *entry += 1;

                let done = pos != vent.b;

                // move to the next pos
                if vent.a.y < vent.b.y {
                    pos.y += 1;
                } else if vent.a.y > vent.b.y {
                    pos.y -= 1;
                }
                if vent.a.x < vent.b.x {
                    pos.x += 1;
                } else if vent.a.x > vent.b.x {
                    pos.x -= 1;
                }

                done
            } {}
        }
    }

    #[allow(unused)]
    fn draw_map(&mut self, width: i32) {
        for y in 0..width {
            for x in 0..width {
                let coord = Coords { x, y };
                if let Some(val) = self.map.get_key_value(&coord) {
                    print!("{}", val.1);
                } else {
                    print!("-");
                }
            }
            print!("\n");
        }
    }

    fn puzzle1(&mut self) -> i32 {
        self.build_map(false);

        let mut above = 0;
        self.map.iter().for_each(|entry| {
            if entry.1 > &1 {
                above += 1
            }
        });
        above
    }

    fn puzzle2(&mut self) -> i32 {
        self.build_map(true);

        // dbg!(&self.map);
        let mut above = 0;
        self.map.iter().for_each(|entry| {
            if entry.1 > &1 {
                above += 1
            }
        });
        above
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
        assert_eq!(data.puzzle1(), 5);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 12);
    }
}
