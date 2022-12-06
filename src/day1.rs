use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut input: Vec<u32> = input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    // sort reverse
    input.sort_by(|a, b| b.cmp(a));
    input
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.get(0).unwrap().to_owned()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    input.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 24000);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 45000);
    }
}
