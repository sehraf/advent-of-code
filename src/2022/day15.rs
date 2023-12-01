use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};
use rayon::prelude::*;

type Pos = (i64, i64);

pub struct Entry {
    sensor: Pos,
    beacon: Pos,
    manhattan_distance: i64,
}

fn calc_manhattan_distance(a: &Pos, b: &Pos) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

#[allow(dead_code)]
fn pretty_print(input: &[Entry]) {
    for y in -3..25 {
        print!("{y:>3} ");
        'X: for x in -5..30 {
            for e in input {
                let pos = (x, y);
                if e.beacon == pos {
                    print!("B");
                    continue 'X;
                } else if e.sensor == pos {
                    print!("S");
                    continue 'X;
                }
            }
            print!(".");
        }
        println!("");
    }
}

#[allow(dead_code)]
fn pretty_print_row(input: &[Entry], y: i64) {
    print!("{y:>3} ");
    'X: for x in -5..30 {
        let pos = (x, y);
        for e in input {
            if calc_manhattan_distance(&pos, &e.sensor) <= e.manhattan_distance {
                if e.beacon == pos {
                    print!("B");
                    continue 'X;
                } else if e.sensor == pos {
                    print!("S");
                    continue 'X;
                } else {
                    print!("#");
                }
                continue 'X;
            }
        }
        print!(".");
    }
    println!("");
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty());
    ret
}

#[aoc(day15, part1)]
pub fn part1(input: &[Entry]) -> usize {
    #[cfg(test)]
    const Y: i64 = 10;
    #[cfg(not(test))]
    const Y: i64 = 2000000;

    #[cfg(test)]
    pretty_print(input);

    let (range, candidates) = {
        let mut min = 0;
        let mut max = 0;
        let mut candidates = vec![];

        for e in input {
            min = min.min(e.sensor.0 - e.manhattan_distance);
            max = max.max(e.sensor.0 + e.manhattan_distance);

            // filter out sensors that are out of range
            if e.sensor.1.abs_diff(Y) as i64 <= e.manhattan_distance {
                candidates.push(e);
            }
        }
        // min - 1..=max + 1
        (min..=max, candidates)
    };

    #[cfg(test)]
    {
        pretty_print_row(input, Y - 1);
        pretty_print_row(input, Y);
        pretty_print_row(input, Y + 1);
    }

    range
        .into_par_iter()
        .filter(|x| {
            let pos = (*x, Y);
            for e in &candidates {
                if calc_manhattan_distance(&pos, &e.sensor) <= e.manhattan_distance {
                    if e.beacon != pos && e.sensor != pos {
                        return true;
                    }
                    break;
                }
            }
            false
        })
        .count()
}

#[aoc(day15, part2)]
pub fn part2(input: &[Entry]) -> i64 {
    #[cfg(test)]
    const Y: i64 = 20;
    #[cfg(not(test))]
    const Y: i64 = 4_000_000;

    #[cfg(test)]
    const X: i64 = 20;
    #[cfg(not(test))]
    const X: i64 = 4_000_000;

    #[cfg(old)]
    {
        let sets: Vec<_> = input
            .par_iter()
            .map(|e| {
                let mut candidates = vec![];

                let sx = e.sensor.0;
                let sy = e.sensor.1;
                let d = e.manhatten_distance + 1;
                for y in sy - d..sy + d {
                    // is always negative
                    let x = (y - sy).abs() - d;

                    let p1 = (sx + x, y);
                    let p2 = (sx - x, y);

                    // println!("{x},{y}");
                    candidates.push(p1);
                    candidates.push(p2);
                }
                candidates
            })
            .collect();

        'L: for (x, y) in sets.iter().flatten() {
            if *x < 0 || *y < 0 || *x > X || *y > Y {
                continue;
            }

            let pos = (*x, *y);
            for e in input {
                if calc_manhattan_distance(&pos, &e.sensor) <= e.manhatten_distance {
                    continue 'L;
                }
            }

            return x * 4_000_000 + y;
        }

        let (x, y) = sets
            .par_iter()
            .flatten()
            .find_any(|(x, y)| {
                if *x < 0 || *y < 0 || *x > X || *y > Y {
                    false
                } else {
                    let pos = (*x, *y);
                    for e in input {
                        if calc_manhattan_distance(&pos, &e.sensor) <= e.manhatten_distance {
                            return false;
                        }
                    }

                    true
                }
            })
            .unwrap();
        x * 4_000_000 + y
    }
    #[cfg(not(old))]
    {
        // based on https://www.reddit.com/r/adventofcode/comments/zmcn64/comment/j0b90nr/
        let mut acoeffs = HashSet::new();
        let mut bcoeffs = HashSet::new();
        for e in input {
            let x = e.sensor.0;
            let y = e.sensor.1;
            let r = e.manhattan_distance;

            acoeffs.insert(y - x + r + 1);
            acoeffs.insert(y - x - r - 1);
            bcoeffs.insert(x + y + r + 1);
            bcoeffs.insert(x + y - r - 1);
        }

        for a in &acoeffs {
            'L: for b in &bcoeffs {
                if (b - a) % 2 == 1 {
                    continue;
                }

                let p = ((b - a) / 2, (b + a) / 2);
                if p.0 < 0 || p.0 > X || p.1 < 0 || p.1 > Y {
                    continue;
                }

                for e in input {
                    if calc_manhattan_distance(&p, &e.sensor) <= e.manhattan_distance {
                        continue 'L;
                    }
                }
                return p.0 * 4_000_000 + p.1;
            }
        }
    }
    0
}

fn parse_coords(input: &str) -> nom::IResult<&str, Pos> {
    let (rem, x) = preceded(
        tag("x="),
        map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse),
    )(input)?;
    let (rem, _) = tag(", ")(rem)?;
    let (rem, y) = preceded(
        tag("y="),
        map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse),
    )(rem)?;

    Ok((rem, (x, y)))
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Entry>> {
    separated_list1(
        line_ending,
        map(
            separated_pair(
                preceded(tag("Sensor at "), parse_coords),
                tag(": "),
                preceded(tag("closest beacon is at "), parse_coords),
            ),
            |(sensor, beacon)| {
                let manhatten_distance = calc_manhattan_distance(&sensor, &beacon);
                Entry {
                    sensor,
                    beacon,
                    manhattan_distance: manhatten_distance,
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 56000011);
    }
}
