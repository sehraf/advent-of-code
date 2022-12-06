use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::Finish;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::sequence::separated_pair;

pub struct Data {
    a: (u32, u32),
    b: (u32, u32),
}

impl Data {
    fn overlapping_p1(&self) -> bool {
        match (self.a, self.b) {
            (a, b) if a.0 <= b.0 && a.1 >= b.1 => true,
            (a, b) if a.0 >= b.0 && a.1 <= b.1 => true,
            _ => false,
        }
    }

    fn overlapping_p2(&self) -> bool {
        match (self.a, self.b) {
            (a, b) if a.0 >= b.0 && a.0 <= b.1 => true,
            (a, b) if a.1 >= b.0 && a.1 <= b.1 => true,
            (b, a) if a.0 >= b.0 && a.0 <= b.1 => true,
            (b, a) if a.1 >= b.0 && a.1 <= b.1 => true,
            _ => false,
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| parse(line).finish().map(|(_, x)| x).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Data]) -> u32 {
    input.iter().filter(|d| d.overlapping_p1()).count() as u32
}

#[aoc(day4, part2)]
pub fn part2(input: &[Data]) -> u32 {
    input.iter().filter(|d| d.overlapping_p2()).count() as u32
}

fn parse_range(input: &str) -> nom::IResult<&str, (u32, u32)> {
    separated_pair(
        map_res(digit1, str::parse),
        tag("-"),
        map_res(digit1, str::parse),
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, Data> {
    map(
        separated_pair(parse_range, tag(","), parse_range),
        |(a, b)| Data { a, b },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 4);
    }
}
