use std::{array::from_mut, cmp::Ordering};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
};

#[derive(Debug, Clone, Eq)]
pub enum Entry {
    Num(u32),
    List(Vec<Entry>),
}

impl Entry {
    pub fn right_order(&self, other: &Self) -> Ordering {
        use Entry::*;

        // println!("comparing: {self:?} vs {other:?}");

        match (self, other) {
            // both number
            (Num(a), Num(b)) if a < b => Ordering::Less,
            (Num(a), Num(b)) if a == b => Ordering::Equal,
            (Num(a), Num(b)) if a > b => Ordering::Greater,

            // both list
            (List(a), List(b)) => {
                for (a, b) in a.iter().zip(b) {
                    match a.right_order(b) {
                        Ordering::Equal => continue,
                        o @ _ => return o,
                    }
                }
                match (a.len(), b.len()) {
                    (a, b) if a < b => Ordering::Less,
                    (a, b) if a == b => Ordering::Equal,
                    (a, b) if a > b => Ordering::Greater,
                    (_, _) => unreachable!(),
                }
            }

            // needs wrapping
            (a, Num(n)) => {
                let b = Entry::List(vec![Entry::Num(*n)]);
                a.right_order(&b)
            }
            (Num(n), b) => {
                let a = Entry::List(vec![Entry::Num(*n)]);
                a.right_order(b)
            }
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.right_order(other)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.right_order(other) == Ordering::Equal
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(Entry, Entry)> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty());
    ret
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Entry, Entry)]) -> usize {
    input
        .iter()
        .map(|(a, b)| a.right_order(b))
        .enumerate()
        .filter(|(_, b)| matches!(b, Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Entry, Entry)]) -> usize {
    let div_a = Entry::List(vec![Entry::List(vec![Entry::Num(2)])]);
    let div_b = Entry::List(vec![Entry::List(vec![Entry::Num(6)])]);

    let mut frames: Vec<Entry> = vec![div_a.to_owned(), div_b.to_owned()];
    frames.extend(
        input
            .iter()
            .map(|(a, b)| vec![a.to_owned(), b.to_owned()])
            .flatten(),
    );

    frames.sort();
    // println!("{frames:#?}");

    frames
        .into_iter()
        .enumerate()
        .filter(|(_, e)| *e == div_a || *e == div_b)
        .map(|(i, _)| i + 1)
        .reduce(|acc, b| acc * b)
        .unwrap()
}

fn parse_entry(input: &str) -> nom::IResult<&str, Entry> {
    nom::branch::alt((
        map(map_res(digit1, str::parse), |d| Entry::Num(d)),
        map(tag("[]"), |_| Entry::List(vec![])),
        map(
            delimited(tag("["), separated_list1(tag(","), parse_entry), tag("]")),
            |l| Entry::List(l),
        ),
    ))(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(Entry, Entry)>> {
    separated_list1(
        pair(line_ending, line_ending),
        separated_pair(parse_entry, line_ending, parse_entry),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 140);
    }
}
