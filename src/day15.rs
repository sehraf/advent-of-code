use std::{cmp::Reverse, collections::BinaryHeap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day15";

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    parent: Option<(usize, usize)>,
    cost: Option<u64>,
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Vec<(u64, Node)>>,
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
        self.input = lines
            .map(|l| {
                l.chars()
                    .map(|c| {
                        (
                            c.to_digit(10).unwrap() as u64,
                            Node {
                                ..Default::default()
                            },
                        )
                    })
                    .collect()
            })
            .collect();
    }

    fn puzzle1(&mut self) -> u64 {
        self.calc_costs();
        self.input.last().unwrap().last().unwrap().1.cost.unwrap()
    }

    fn puzzle2(&mut self) -> u64 {
        self.zoom_out();
        self.calc_costs();
        self.input.last().unwrap().last().unwrap().1.cost.unwrap()
    }

    fn calc_costs(&mut self) {
        let max_pos = (self.input[0].len(), self.input.len());

        // start
        let start = &mut self.input[0][0].1;
        start.cost = Some(0);

        // initial candidates
        let mut candidates = BinaryHeap::new();
        for (x, y) in get_directions(&(0, 0), &max_pos) {
            // use reverse since we want to `pop()` the small values first
            candidates.push(Reverse((self.input[y][x].0, (x, y))))
        }

        // is this A* ?
        while let Some(Reverse((cost, (x, y)))) = candidates.pop() {
            for (x_2, y_2) in get_directions(&(x, y), &max_pos) {
                let candidate = &mut self.input[y_2][x_2];
                let cost_2 = cost + candidate.0;

                if candidate.1.cost.is_none() {
                    // not visited before
                    assert_eq!(candidate.1.parent, None);

                    candidate.1.parent = Some((x, y));
                    candidate.1.cost = Some(cost_2);
                    candidates.push(Reverse((cost_2, (x_2, y_2))));
                    continue;
                } else if candidate.1.cost.unwrap() > cost_2 {
                    // visited before but now shorter
                    assert_ne!(candidate.1.parent, Some((x, y)));

                    candidate.1.parent = Some((x, y));
                    candidate.1.cost = Some(cost_2);
                    candidates.push(Reverse((cost_2, (x_2, y_2))));
                    continue;
                } else {
                    // visited before and not shorter
                    if x_2 == max_pos.0 && y_2 == max_pos.1 {
                        // are we there yet?
                        break;
                    }
                    continue;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn plot(&self) {
        for y in &self.input {
            for x in y {
                // prints costs for path
                // if let Some(c) = x.1.cost {
                //     print!("{}", c % 10);
                // } else {
                //     print!(".");
                // }

                // prints costs for node
                print!("{}", x.0);
            }
            println!()
        }
    }

    fn zoom_out(&mut self) {
        let old_size = (self.input[0].len(), self.input.len());

        // rows
        for i in 0..4 {
            for j in 0..old_size.1 {
                self.input.push(
                    self.input[j + i * old_size.1]
                        .iter()
                        .map(|(cost, node)| {
                            let mut cost = cost.clone() + 1;
                            let node = node.to_owned();
                            if cost > 9 {
                                cost -= 9;
                            }
                            (cost, node)
                        })
                        .collect(),
                );
            }
        }

        // columns
        for row in &mut self.input {
            let mut append = vec![];
            for i in 0..4 {
                row.iter().for_each(|(cost, node)| {
                    append.push({
                        let mut cost = cost.clone() + 1 + i;
                        let node = node.to_owned();
                        if cost > 9 {
                            cost -= 9;
                        }
                        (cost, node)
                    })
                });
            }

            row.append(&mut append);
        }
    }
}

fn get_directions(pos: &(usize, usize), max_pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut dirs = vec![];
    if pos.0 > 1 {
        dirs.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 1 {
        dirs.push((pos.0, pos.1 - 1));
    }
    if pos.0 < max_pos.0 - 1 {
        dirs.push((pos.0 + 1, pos.1));
    }
    if pos.1 < max_pos.1 - 1 {
        dirs.push((pos.0, pos.1 + 1));
    }
    dirs
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
        assert_eq!(data.puzzle1(), 40);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 315);
    }
}
