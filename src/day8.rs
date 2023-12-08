use num::integer::lcm;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Task {
    dir: Vec<bool>, // right = true
    map: HashMap<String, (String, String)>,
}

type T = Task;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<T> {
    let mut lines = input.lines();
    let dir = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => true,
            'L' => false,
            _ => unreachable!(),
        })
        .collect();
    assert!(lines.next().unwrap().is_empty());

    let map = lines
        .map(|line| {
            let (a, b) = line.split_once('=').unwrap();
            let (b, c) = b.split_once(',').unwrap();

            // some sanity checks
            debug_assert!(a.ends_with(' '));
            debug_assert!(b.starts_with(" ("));
            debug_assert!(c.ends_with(')'));

            let b = b[2..].to_owned();
            let c = c[1..4].to_owned();

            (a.trim().to_owned(), (b, c))
        })
        .collect();

    vec![Task { dir, map }]
}

fn step<'a>(input: &'a Task, pos: &str, counter: usize) -> &'a str {
    let dir = input.dir.get(counter % input.dir.len()).unwrap();
    let options = input.map.get(pos).unwrap();
    let new_pos = if *dir {
        options.1.as_str()
    } else {
        options.0.as_str()
    };
    new_pos
}

#[aoc(day8, part1)]
pub fn part1(input: &[T]) -> u32 {
    let input = input.first().unwrap();

    let start = String::from("AAA");
    let end = String::from("ZZZ");

    let mut pos = start.as_str();
    let mut counter = 0;

    while pos != end {
        pos = step(input, &pos, counter);
        counter += 1;
    }

    counter as u32
}

#[aoc(day8, part2)]
pub fn part2(input: &[T]) -> u64 {
    let input = input.first().unwrap();

    let a = input.map.keys().filter(|k| k.ends_with('A')).count();
    let z = input.map.keys().filter(|k| k.ends_with('Z')).count();
    assert_eq!(a, z);

    input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| {
            let mut pos = k.as_str();
            let mut counter = 0;

            while !pos.ends_with('Z') {
                pos = step(input, &pos, counter);
                counter += 1;
            }

            counter as u64
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 6);
    }

    #[test]
    fn test2() {
        // 3228318959 too low
        assert_eq!(part2(&input_generator(INPUT2)), 6);
    }
}
