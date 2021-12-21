use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
};

use crate::AdventOfCode;

const DAY: &str = "day12";

#[derive(Debug, Default, PartialEq)]
struct Cave {
    name: String,
    connections: Vec<String>,
    small: bool,
}

#[derive(Debug, Default)]
pub struct Data {
    input: HashMap<String, Cave>,
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

        // collect information
        let mut caves = HashSet::new();
        let mut connections = vec![];
        lines.for_each(|line| {
            let mut cs = line.split_terminator('-');
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            caves.insert(a.to_owned());
            caves.insert(b.to_owned());
            connections.push((a.to_owned(), b.to_owned()));
        });

        // store data
        self.input = caves
            .into_iter()
            .map(|name| {
                (
                    name.to_owned(),
                    Cave {
                        name: name.to_owned(),
                        connections: vec![],
                        small: name.to_uppercase() != name,
                    },
                )
            })
            .collect();

        for (a, b) in connections {
            self.input
                .get_mut(&a)
                .unwrap()
                .connections
                .push(b.to_owned());
            self.input
                .get_mut(&b)
                .unwrap()
                .connections
                .push(a.to_owned());
        }
    }

    fn puzzle1(&mut self) -> u64 {
        let s = String::from("start");
        let start = self.input.get(&s).unwrap();

        let paths = self.visit_connections_flat(start, false);
        paths.len() as u64
    }

    fn puzzle2(&mut self) -> u64 {
        let s = String::from("start");
        let start = self.input.get(&s).unwrap();

        let paths = self.visit_connections_flat(start, true);
        paths.len() as u64
    }

    #[allow(dead_code)]
    fn visit_connections_recursive(
        &self,
        node: &Cave,
        previous_path: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut visits = vec![];
        for con in &node.connections {
            let small = &con.to_owned().to_uppercase() != con;

            // small caves check
            if small && previous_path.contains(con) {
                continue;
            }

            // the end?
            if con == "end" {
                let mut p = previous_path.to_owned();
                p.push(con.to_owned());
                visits.push(p);
                continue;
            }

            // visit connections
            let new_node = self.input.get(con).unwrap();
            let mut new_path = previous_path.to_owned();
            new_path.push(con.to_owned());
            let new_paths = self.visit_connections_recursive(new_node, new_path);

            for mut new_path in new_paths {
                let mut p = previous_path.to_owned();
                p.push(con.to_owned());
                p.append(&mut new_path);
                visits.push(p);
            }
        }

        visits
    }

    fn visit_connections_flat<'a>(
        &'a self,
        start: &'a Cave,
        task_two_condition: bool,
    ) -> Vec<Vec<&'a Cave>> {
        let mut paths = vec![];
        let mut next_candidates = VecDeque::new();

        next_candidates.push_back((start, vec![]));

        while let Some((current_node, previous_path)) = next_candidates.pop_front() {
            for next_node_name in &self.input.get(&current_node.name).unwrap().connections {
                let next_node = self.input.get(next_node_name).unwrap();

                // end
                if next_node.name == "end" {
                    let mut pp = previous_path.to_owned();
                    pp.push(current_node);
                    pp.push(next_node);
                    paths.push(pp);
                    continue;
                }

                // is lower case?
                if next_node.small {
                    if task_two_condition {
                        // start
                        if next_node.name == "start" {
                            continue;
                        }

                        // filter small caves
                        let mut small_caves: Vec<&&Cave> =
                            previous_path.iter().filter(|c| c.small).collect();

                        // add current cave (covers the case where the current cave it the second visit of a small cave)
                        if current_node.small {
                            small_caves.push(&current_node);
                        }

                        // count visits
                        let tmp_p: Vec<usize> = small_caves
                            .iter()
                            .map(|a| small_caves.iter().filter(|b| b.name == a.name).count())
                            .collect();

                        let can_double_visit_small = !tmp_p.iter().any(|f| f >= &2);

                        if previous_path.contains(&next_node) && !can_double_visit_small {
                            continue;
                        }
                    } else {
                        // small caves check
                        if previous_path.contains(&next_node) {
                            continue;
                        }
                    }
                }

                let mut pp = previous_path.to_owned();
                pp.push(current_node);
                next_candidates.push_back((next_node, pp));
            }
        }

        paths
    }
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::{Data, DAY};

    #[test]
    fn puzzle1_1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test_2.txt");
        assert_eq!(data.puzzle1(), 19);
    }

    #[test]
    fn puzzle1_2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test_3.txt");
        assert_eq!(data.puzzle1(), 226);
    }

    #[test]
    fn puzzle2_1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test_1.txt");
        assert_eq!(data.puzzle2(), 36);
    }

    #[test]
    fn puzzle2_2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test_2.txt");
        assert_eq!(data.puzzle2(), 103);
    }

    #[test]
    fn puzzle2_3() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test_3.txt");
        assert_eq!(data.puzzle2(), 3509);
    }
}
