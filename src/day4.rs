use aoc_runner_derive::{aoc, aoc_generator};

type T = u32;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<T> {
    vec![]
}

#[aoc(day4, part1)]
pub fn part1(input: &[T]) -> u32 {
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &[T]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "TODO";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }
}
