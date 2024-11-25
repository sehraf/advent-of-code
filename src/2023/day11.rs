use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// (0,0) is top left
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

pub struct Input {
    unexpanded: Vec<Pos>,
    max_x: usize,
    max_y: usize,
}

type T = Input;
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<T> {
    let unexpanded: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Pos {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().len();

    vec![Input {
        unexpanded,
        max_x,
        max_y,
    }]
}

fn expand(input: &[T], factor: i32) -> Vec<Pos> {
    let input = input.first().unwrap();
    let Input {
        unexpanded,
        max_x,
        max_y,
    } = input;

    let empty_rows: Vec<_> = (0..*max_y)
        .into_iter()
        .filter(|y| !unexpanded.iter().any(|pos| pos.y == *y as i64))
        .collect();
    let empty_columns: Vec<_> = (0..*max_x)
        .into_iter()
        .filter(|x| !unexpanded.iter().any(|pos| pos.x == *x as i64))
        .collect();

    let mut dst_x;
    let mut dst_y = 0;

    let mut result = vec![];

    for src_y in 0..*max_y {
        if empty_rows.contains(&src_y) {
            dst_y += factor - 1;
        }

        dst_x = 0;
        for src_x in 0..*max_x {
            if empty_columns.contains(&src_x) {
                dst_x += factor - 1;
            }

            let src = Pos {
                x: src_x as i64,
                y: src_y as i64,
            };
            let dst = Pos {
                x: dst_x as i64,
                y: dst_y as i64,
            };

            if unexpanded.contains(&src) {
                result.push(dst)
            }

            dst_x += 1;
        }

        dst_y += 1;
    }

    result
}

#[aoc(day11, part1)]
pub fn part1(input: &[T]) -> u64 {
    let map = expand(input, 2);

    let result: u64 = map
        .iter()
        .cartesian_product(map.iter())
        // .filter(|(a, b)| a != b)
        .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
        .sum();

    // we count double: e.g. pair 1+9 and pair 9+1
    result / 2
}

#[aoc(day11, part2)]
pub fn part2(input: &[T]) -> u64 {
    let map = expand(input, 1000000);

    let result: u64 = map
        .iter()
        .cartesian_product(map.iter())
        // .filter(|(a, b)| a != b)
        .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
        .sum();

    // we count double: e.g. pair 1+9 and pair 9+1
    result / 2
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{input_generator, part1, part2};

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 374);
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(part2(&input_generator(INPUT)), 0);
    // }
}
