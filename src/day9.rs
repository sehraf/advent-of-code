use std::{collections::HashMap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day9";

type Basin = u32;

#[derive(Debug, Default)]
pub struct Data {
    input: HashMap<(i32, i32), (i32, Option<Basin>)>,
    width: i32,
    height: i32,
    next_basin: Basin,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1() as u64;

        // self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle2() as u64;

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
        let mut width = -1;
        let mut height = 0;
        while let Some(next_line) = lines.next() {
            if width == -1 {
                width = next_line.len() as i32;
            }
            let mut x = 0;
            self.input.extend(next_line.chars().map(|c| {
                let key = (x, height);
                let val = (c.to_digit(10).unwrap() as i32, None);
                x += 1;
                (key, val)
            }));
            assert_eq!(x, width);
            height += 1;
        }

        self.width = width;
        self.height = height;
    }

    fn puzzle1(&mut self) -> i32 {
        self.input
            .iter()
            .filter(|entry| {
                let pos = entry.0;
                let height = entry.1 .0;
                self.local_minimum(height, pos)
            })
            .map(|entry| entry.1 .0 + 1)
            .sum()
    }

    fn puzzle2(&mut self) -> i32 {
        self.match_basins();
        let mut basins_count = HashMap::new();
        for basin in 0..self.next_basin {
            basins_count.insert(
                basin,
                self.input
                    .iter()
                    .filter(|elem| elem.1 .1 == Some(basin))
                    .count(),
            );
        }

        let mut sum = 1;
        for _ in 0..3 {
            let max_entry = basins_count.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            // needed for last line
            let max_entry = (max_entry.0.to_owned(), max_entry.1.to_owned());
            sum *= max_entry.1 as i32;
            basins_count.remove(&max_entry.0);
        }
        sum
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.input.get(&(x, y)).unwrap().0 {
                    0 | 1 | 2 => print!("."),
                    3 | 4 | 5 => print!(":"),
                    6 | 7 | 8 => print!("a"),
                    9 => print!("#"),
                    _ => unreachable!(),
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_basin(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.input.get(&(x, y)).unwrap().1 {
                    None => print!(" "),
                    Some(val) => print!("{}", val % 10),
                }
            }
            println!();
        }
    }

    fn match_basins(&mut self) {
        // solution: rooted trees!
        for y in 0..self.height {
            for x in 0..self.width {
                if self.input.get_mut(&(x, y)).unwrap().1.is_some() {
                    continue;
                }
                // find existing basin or creat one
                let (basin, way) = self.march_down(&(x, y));
                // mark walk
                for steps in way {
                    self.input.get_mut(&steps).unwrap().1 = basin;
                }
            }
        }
    }

    fn march_down(&mut self, start_pos: &(i32, i32)) -> (Option<Basin>, Vec<(i32, i32)>) {
        let height = self.input.get(start_pos).unwrap();
        if height.0 == 9 {
            return (None, vec![]);
        }
        assert!(height.1.is_none());

        let mut height = height.0;
        let mut pos = start_pos.to_owned();
        let mut way = vec![pos];
        loop {
            // bottom?
            if self.local_minimum(height, &pos) {
                let ret = self.next_basin;
                self.next_basin += 1;
                return (Some(ret), way);
            }

            for new_pos in [
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
            ] {
                if let Some(neighbor) = self.input.get(&new_pos) {
                    if neighbor.1.is_some() {
                        return (neighbor.1, way);
                    }
                    if neighbor.0 < height {
                        pos = new_pos;
                        way.push(new_pos);
                        height = neighbor.0;
                    }
                }
            }
        }
    }

    fn local_minimum(&self, height: i32, pos: &(i32, i32)) -> bool {
        self.input.get(&(pos.0 + 1, pos.1)).unwrap_or(&(10, None)).0 > height
            && self.input.get(&(pos.0, pos.1 + 1)).unwrap_or(&(10, None)).0 > height
            && self.input.get(&(pos.0 - 1, pos.1)).unwrap_or(&(10, None)).0 > height
            && self.input.get(&(pos.0, pos.1 - 1)).unwrap_or(&(10, None)).0 > height
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
        assert_eq!(data.puzzle1(), 15);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 1134);
    }
}
