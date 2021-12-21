use itertools::Itertools;
use std::{collections::HashSet, path::PathBuf, slice::Iter, vec};

use crate::AdventOfCode;

const DAY: &str = "day19";

type Pos = [i64; 3];

#[derive(Debug, Default)]
struct Scanner {
    orientation: u8,
    beacons: Vec<Pos>,
    position: Option<Pos>,
}

#[derive(Clone)]
struct ScannerRotIt<'a> {
    rot: u8,
    it_internal: Iter<'a, Pos>,
}

impl<'a> ScannerRotIt<'a> {
    fn new(scanner: &'a Scanner, rot: u8) -> ScannerRotIt<'a> {
        ScannerRotIt {
            rot,
            it_internal: scanner.beacons.iter(),
        }
    }
}

impl<'a> Iterator for ScannerRotIt<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let [x, y, z] = self.it_internal.next()?.to_owned();
        Some(match self.rot {
            0 => [x, y, z],
            1 => [y, -x, z],
            2 => [-x, -y, z],
            3 => [-y, x, z],
            4 => [z, y, -x],
            5 => [y, -z, -x],
            6 => [-z, -y, -x],
            7 => [-y, z, -x],
            8 => [z, -x, -y],
            9 => [-x, -z, -y],
            10 => [-z, x, -y],
            11 => [x, z, -y],
            12 => [z, -y, x],
            13 => [-y, -z, x],
            14 => [-z, y, x],
            15 => [y, z, x],
            16 => [z, x, y],
            17 => [x, -z, y],
            18 => [-z, -x, y],
            19 => [-x, z, y],
            20 => [-x, y, -z],
            21 => [y, x, -z],
            22 => [x, -y, -z],
            23 => [-y, -x, -z],
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Scanner>,
    known_beacons: HashSet<Pos>,
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
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let lines = input.lines();
        let mut beacons = vec![];
        self.input.clear();
        lines.filter(|line| !line.is_empty()).for_each(|line| {
            if line.contains("scanner") {
                if !beacons.is_empty() {
                    self.input.push(Scanner {
                        orientation: 0,
                        beacons: beacons.to_owned(),
                        position: (None),
                    })
                }
                beacons.clear();
            } else {
                let mut parts = line.split_terminator(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                let z = parts.next().unwrap().parse().unwrap();
                beacons.push([x, y, z]);
            }
        });
        self.input.push(Scanner {
            orientation: 0,
            beacons: beacons.to_owned(),
            position: (None),
        });
    }

    fn puzzle1(&mut self) -> u64 {
        // we need to start somewhere
        self.known_beacons.clear();
        self.known_beacons
            .extend(self.input.first().unwrap().beacons.to_owned().into_iter());

        self.find_mapping();

        self.known_beacons.len() as u64
    }

    fn puzzle2(&mut self) -> u64 {
        #[cfg(test)]
        self.puzzle1();

        let manhattan = |a: Pos, b: Pos| -> i64 {
            (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
        };

        self.input
            .iter()
            .tuple_combinations()
            .map(|(a, b)| manhattan(a.position.unwrap(), b.position.unwrap()))
            .max()
            .unwrap() as u64
    }

    fn find_mapping(&mut self) {
        while self.input.iter().any(|s| s.position.is_none()) {
            self.input
                .iter_mut()
                .filter(|s| s.position.is_none())
                .for_each(|s| {
                    (0..24).find_map(|rot| diff_scans(&mut self.known_beacons, s, rot));
                });
        }
    }
}

fn diff_scans(ground_truth: &mut HashSet<Pos>, b: &mut Scanner, rot: u8) -> Option<(u8, Pos)> {
    let it = ScannerRotIt::new(b, rot);

    let diffs: Vec<Pos> = ground_truth
        .iter()
        .cartesian_product(it.to_owned())
        .map(|([ax, ay, az], [bx, by, bz])| [ax - bx, ay - by, az - bz])
        .collect();
    for [dx, dy, dz] in diffs {
        let shifted = it.to_owned().map(|[x, y, z]| [x + dx, y + dy, z + dz]);
        if shifted
            .clone()
            .filter(|corrected_pos| ground_truth.contains(corrected_pos))
            .count()
            >= 12
        {
            ground_truth.extend(shifted);
            b.orientation = rot;
            b.position = Some([dx, dy, dz]);
            return Some((rot, [dx, dy, dz]));
        }
    }

    None
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
        assert_eq!(data.puzzle1(), 79);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 3621);
    }
}
