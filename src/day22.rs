use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::{eof, map, recognize},
    sequence::{delimited, separated_pair, terminated, tuple},
    Finish,
};
use std::{collections::HashMap, path::PathBuf};

use crate::AdventOfCode;

const DAY: &str = "day22";

type Edge = (i64, i64);
type Cube = (Edge, Edge, Edge);
struct Cubus {
    on: bool,
    cube: Cube,
}

type Instruction = (bool, Cube);

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Instruction>,
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
        self.input = lines
            .map(|line| {
                parse(line)
                    .finish()
                    .map(|(_, b)| b)
                    .map_err(|e| nom::error::Error::new(e.input.to_owned(), e.code))
                    .unwrap()
            })
            .collect();
    }

    fn puzzle1(&mut self) -> u64 {
        let mut reactor: HashMap<(i64, i64, i64), bool> = HashMap::new();

        // it DOES work ...
        for &(on, ((x1, x2), (y1, y2), (z1, z2))) in &self.input {
            if x1 < -50 || x2 > 50 || y1 < -50 || y2 > 50 || z1 < -50 || z2 > 50 {
                // kind of overkill but works/passes
                continue;
            }
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        *reactor.entry((x, y, z)).or_default() = on;
                    }
                }
            }
        }

        reactor.iter().filter(|(_, &on)| on).count() as u64
    }

    fn puzzle2(&mut self) -> u64 {
        let mut all_cubi = Vec::<Cubus>::new();
        for (on, cube) in &self.input {
            let mut new_cubi = Vec::new();
            if *on {
                new_cubi.push(Cubus {
                    on: true,
                    cube: *cube,
                });
            }
            for c in all_cubi.iter() {
                if let Some(collision) = intersection(cube, &c.cube) {
                    new_cubi.push(Cubus {
                        // collisions will be counted twice, add negated intersection
                        on: !c.on,
                        cube: collision,
                    });
                }
            }
            all_cubi.extend(new_cubi);
        }
        all_cubi
            .iter()
            .map(|c| {
                let sign = if c.on { 1 } else { -1 };
                let (x, y, z) = c.cube;
                sign * ((x.1 - x.0) + 1) * ((y.1 - y.0) + 1) * ((z.1 - z.0) + 1)
            })
            .sum::<i64>() as u64
    }
}

fn intersection(left: &Cube, right: &Cube) -> Option<Cube> {
    let c = (
        (left.0 .0.max(right.0 .0), left.0 .1.min(right.0 .1)),
        (left.1 .0.max(right.1 .0), left.1 .1.min(right.1 .1)),
        (left.2 .0.max(right.2 .0), left.2 .1.min(right.2 .1)),
    );
    if c.0 .0 <= c.0 .1 && c.1 .0 <= c.1 .1 && c.2 .0 <= c.2 .1 {
        Some(c)
    } else {
        None
    }
}

fn parse(line: &str) -> nom::IResult<&str, Instruction> {
    // on x=-49..1,y=-3..46,z=-24..28
    // off x=-48..-32,y=26..41,z=-47..-37

    tuple((
        map(
            terminated(alt((recognize(tag("on")), recognize(tag("off")))), space1),
            |s| s == "on",
        ),
        tuple((
            delimited(tag("x="), separated_pair(i64, tag(".."), i64), tag(",")),
            delimited(tag("y="), separated_pair(i64, tag(".."), i64), tag(",")),
            delimited(tag("z="), separated_pair(i64, tag(".."), i64), eof),
        )),
    ))(line)
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
        assert_eq!(data.puzzle1(), 474140);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 2758514936282235);
    }
}
