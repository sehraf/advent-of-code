use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::pair,
};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut input = input.to_owned();
    input.sort_by(|a, b| b.cmp(a));
    input.get(0).unwrap().to_owned()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut input = input.to_owned();
    input.sort_by(|a, b| b.cmp(a));
    input.iter().take(3).sum()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<u32>> {
    separated_list1(
        pair(line_ending, line_ending),
        map(
            separated_list1(line_ending, map_res(digit1, str::parse::<u32>)),
            |l| l.into_iter().sum(),
        ),
    )(input)
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
