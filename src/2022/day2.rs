use std::collections::HashMap;

use aoc_runner_derive::aoc;
// use aoc_runner_derive::aoc_generator;

// #[aoc_generator(day2)]
// pub fn input_generator(input: &str) -> Vec<u32> {
// }

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won)
    let x = 1;
    let y = 2;
    let z = 3;

    let w = 6;
    let d = 3;
    let l = 0;

    let scores: HashMap<&str, u32> = [
        ("A X", x + d),
        ("A Y", y + w),
        ("A Z", z + l),
        ("B X", x + l),
        ("B Y", y + d),
        ("B Z", z + w),
        ("C X", x + w),
        ("C Y", y + l),
        ("C Z", z + d),
    ]
    .into_iter()
    .collect();

    input
        .lines()
        .map(|l| scores.get(l).expect("combination not found").to_owned())
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    // A for Rock, B for Paper, and C for Scissors
    // X to lose, Y to draw, and Z to win
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won)
    let x = 0;
    let y = 3;
    let z = 6;

    let r = 1;
    let p = 2;
    let s = 3;

    let scores: HashMap<&str, u32> = [
        ("A X", x + s),
        ("A Y", y + r),
        ("A Z", z + p),

        ("B X", x + r),
        ("B Y", y + p),
        ("B Z", z + s),

        ("C X", x + p),
        ("C Y", y + s),
        ("C Z", z + r),
    ]
    .into_iter()
    .collect();

    input
        .lines()
        .map(|l| scores.get(l).expect("combination not found").to_owned())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test1() {
        assert_eq!(part1(&INPUT), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&INPUT), 12);
    }
}
