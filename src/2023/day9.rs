use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type T = Vec<Vec<i64>>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|history| {
            let mut diffs = vec![];
            diffs.push(history.to_owned());

            // 0   3   6   9  12  15
            //   3   3   3   3   3
            //     0   0   0   0
            while diffs.last().unwrap().iter().any(|a| *a != 0) {
                let diff = diffs
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();

                diffs.push(diff);
            }
            diffs
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[T]) -> i64 {
    input
        .iter()
        .map(|diffs| {
            // 0   3   6   9  12  15   B
            //   3   3   3   3   3   A
            //     0   0   0   0   0
            diffs.iter().rev().fold(0, |a, b| a + b.last().unwrap())
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[T]) -> i64 {
    input
        .iter()
        .map(|diffs| {
            // 5  10  13  16  21  30  45
            //   5   3   3   5   9  15
            //    -2   0   2   4   6
            //       2   2   2   2
            //         0   0   0
            diffs.iter().rev().fold(0, |a, b| b.first().unwrap() - a)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 114);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 2);
    }
}
