use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

fn convert_to_hashset(input: &str) -> HashSet<u32> {
    // println!("{input:?}");

    HashSet::from_iter(
        input
            .chars()
            .map(|c| match c as u8 {
                c if c >= b'a' && c <= b'z' => c - b'a' + 1,
                c if c >= b'A' && c <= b'Z' => c - b'A' + 1 + 26,
                _ => panic!("unexpected character"),
            })
            .map(|c| c as u32),
    )
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            assert_eq!(a.len(), b.len(), "number of rucksack items is odd");
            (a, b)
        })
        .map(|(a, b)| (convert_to_hashset(a), convert_to_hashset(b)))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    // println!("{input:#?}");
    input
        .iter()
        .map(|(a, b)| a.intersection(b).collect::<Vec<_>>())
        .map(|i| {
            // we only expect one common item
            assert_eq!(i.len(), 1, "found multiple common items");
            *i.first().unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    input
        .into_iter()
        .map(|(a, b)| HashSet::from_iter(a.union(b)))
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let folded = chunk
                .into_iter()
                .reduce(|acc: HashSet<&u32>, c| {
                    HashSet::from_iter(acc.intersection(&c).map(|a| *a))
                })
                .unwrap();

            // convert to vec to get first (only) element
            let i = folded.into_iter().collect::<Vec<_>>();
            assert_eq!(i.len(), 1, "found multiple common items");
            *i.first().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 157);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 70);
    }
}
