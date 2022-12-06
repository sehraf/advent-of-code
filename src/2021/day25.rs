use std::{collections::HashMap, path::PathBuf};

// use nom::{
//     branch::alt,
//     bytes::complete::tag,
//     character::complete::one_of,
//     combinator::map_res,
//     sequence::{preceded, tuple},
//     Finish,
// };

use crate::AdventOfCode;

const DAY: &str = "day25";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapPosition {
    Empty,
    East,
    South,
}

#[derive(Debug, Default)]
pub struct Data {
    input: HashMap<(usize, usize), MapPosition>,
    max_x: usize,
    max_y: usize,
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
        let tmp: Vec<Vec<MapPosition>> = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'v' => MapPosition::South,
                        '>' => MapPosition::East,
                        '.' => MapPosition::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        self.input.clear();
        self.max_x = tmp[0].len();
        self.max_y = tmp.len();
        for y in 0..tmp.len() {
            for x in 0..tmp[y].len() {
                self.input.insert((x, y), tmp[y][x].to_owned());
            }
        }
    }

    fn puzzle1(&mut self) -> u64 {
        let mut i = 1;

        while self.step() {
            i += 1;
        }
        i
    }

    fn puzzle2(&mut self) -> u64 {
        0
    }

    fn step(&mut self) -> bool {
        let mut progress = false;

        // east
        let mut new_map = HashMap::new();
        for (pos, cu) in &self.input {
            // println!("@ > {:?}", pos);
            match *cu {
                MapPosition::East => {
                    // try move
                    let pos_new = self.next_east_pos(pos);
                    // print!("testing {:?} ", pos_new);
                    match self.input.get(&pos_new) {
                        None | Some(MapPosition::Empty) => {
                            // println!("empty!");
                            progress = true;
                            new_map.insert(pos_new, cu.to_owned());
                        }
                        _ => {
                            // println!("not empty!");
                            new_map.insert(pos.to_owned(), cu.to_owned());
                        }
                    }
                }
                MapPosition::Empty => {}
                MapPosition::South => {
                    // copy
                    let _ = new_map.insert(pos.to_owned(), cu.to_owned());
                }
            }
        }
        self.input = new_map;
        // self.plot();

        // south
        let mut new_map = HashMap::new();
        for (pos, cu) in &self.input {
            match *cu {
                MapPosition::East => {
                    // copy
                    let _ = new_map.insert(pos.to_owned(), cu.to_owned());
                }
                MapPosition::Empty => {}
                MapPosition::South => {
                    // try move
                    let pos_new = self.next_south_pos(pos);
                    match self.input.get(&pos_new) {
                        None | Some(MapPosition::Empty) => {
                            // println!("empty!");
                            progress = true;
                            new_map.insert(pos_new, cu.to_owned());
                        }
                        _ => {
                            // println!("not empty!");
                            new_map.insert(pos.to_owned(), cu.to_owned());
                        }
                    }
                }
            }
        }
        self.input = new_map;

        progress
    }

    fn next_east_pos(&self, pos: &(usize, usize)) -> (usize, usize) {
        let mut p = pos.to_owned();
        p.0 += 1;
        if p.0 >= self.max_x {
            p.0 = 0;
        }
        p
    }

    fn next_south_pos(&self, pos: &(usize, usize)) -> (usize, usize) {
        let mut p = pos.to_owned();
        p.1 += 1;
        if p.1 >= self.max_y {
            p.1 = 0;
        }
        p
    }

    #[allow(dead_code)]
    fn plot(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                match self.input.get(&(x, y)) {
                    None | Some(MapPosition::Empty) => print!("."),
                    Some(MapPosition::East) => print!(">"),
                    Some(MapPosition::South) => print!("v"),
                }
            }
            println!("");
        }
        println!("");
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
        assert_eq!(data.puzzle1(), 58);
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
