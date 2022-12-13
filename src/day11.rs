use std::{collections::VecDeque, fmt::Display, sync::Arc};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair},
};

#[derive(Clone)]
pub struct Monkey {
    num: u32,
    items: VecDeque<u64>,
    new: Arc<dyn Fn(u64) -> u64>,
    test_divisor: u64,
    target_monkeys: (u32, u32), // false, true
}

impl Monkey {
    pub fn handle_one_item(&mut self, bored_factor: u64, modulo: u64) -> Option<(u32, u64)> {
        let item = self.items.pop_front()?;
        let new = (*self.new)(item);
        let bored = (new / bored_factor) % modulo;

        let target = if bored % self.test_divisor == 0 {
            self.target_monkeys.1 // true case
        } else {
            self.target_monkeys.0 // false case
        };

        // println!(
        //     "monkey {}, item {item}, worry {new}, bored {bored}, -> {target}",
        //     self.num
        // );

        Some((target, bored))
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey {}:", self.num)?;
        writeln!(f, "  Starting items: {:?}", self.items)?;
        writeln!(f, "  Operation: new = ...")?;
        writeln!(f, "  Test: divisible by {}", self.test_divisor)?;
        writeln!(f, "    If true: throw to monkey {}", self.target_monkeys.1)?;
        writeln!(f, "    If false: throw to monkey {}", self.target_monkeys.0)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    let (rem, m) = parse(input).unwrap();
    debug_assert!(rem.is_empty());

    m
}

pub struct State {
    monkeys: Vec<Monkey>,
    counter: Vec<u32>,
}

impl State {
    #[allow(dead_code)]
    pub fn print_items(&self) {
        for m in self.monkeys.iter() {
            println!("Monkey {}: {:?}", m.num, m.items);
        }
    }

    #[allow(dead_code)]
    pub fn print_counter(&self) {
        for (m, c) in self.counter.iter().enumerate() {
            println!("Monkey {}: {}", m, c);
        }
    }

    pub fn handle_monkey(&mut self, index: usize, bored_factor: u64, modulo: u64) {
        // handle monkey
        let mut moves = vec![];
        while let Some((target, score)) = self.monkeys[index].handle_one_item(bored_factor, modulo)
        {
            moves.push((target, score));
            self.counter[index] += 1;
        }

        // updates results
        for (target, score) in moves {
            self.monkeys[target as usize].items.push_back(score);
        }
    }
}

fn do_it(input: &[Monkey], rounds: usize, bored_factor: u64) -> u32 {
    let mut counter = vec![];
    for _ in 0..input.len() {
        counter.push(0);
    }

    let modulo = input.iter().fold(1, |acc, monkey| {
        if acc % monkey.test_divisor == 0 {
            acc
        } else {
            acc * monkey.test_divisor
        }
    });

    let mut state = State {
        monkeys: input.to_vec(),
        counter,
    };

    // rounds
    for _round in 1..=rounds {
        for index in 0..state.monkeys.len() {
            state.handle_monkey(index, bored_factor, modulo);
        }

        // if _round % 1_000 == 0 {
        //     println!("Round {_round}");
        //     state.print_counter();
        //     println!("");
        // }
    }

    state.counter.as_mut_slice().sort();
    let end = state.counter.len() - 1;
    state.counter[end] * state.counter[end - 1]
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> u32 {
    do_it(input, 20, 3)
}

#[aoc(day11, part2)]
pub fn part2(input: &[Monkey]) -> u32 {
    do_it(input, 10_000, 1)
}

fn parse_monkey(input: &str) -> nom::IResult<&str, Monkey> {
    let (rem, num) = delimited(
        tag("Monkey "),
        map_res(digit1, str::parse::<u32>),
        tag(":\n"),
    )(input)?;

    let (rem, items) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), map_res(digit1, str::parse)),
        tag("\n"),
    )(rem)?;

    let (rem, op) = delimited(
        tag("  Operation: new = old "),
        map(
            separated_pair(anychar, space1, alphanumeric1),
            |(sign, val)| match (sign, val) {
                ('*', "old") => Arc::new(move |i: u64| -> u64 { i * i }) as Arc<dyn Fn(u64) -> u64>,
                ('+', i) => {
                    let val: u64 = i.parse().unwrap();
                    Arc::new(move |i: u64| -> u64 { i + val }) as Arc<dyn Fn(u64) -> u64>
                }
                ('*', i) => {
                    let val: u64 = i.parse().unwrap();
                    Arc::new(move |i: u64| -> u64 { i * val }) as Arc<dyn Fn(u64) -> u64>
                }
                _ => unimplemented!(),
            },
        ),
        tag("\n"),
    )(rem)?;

    let (rem, divisor) = delimited(
        tag("  Test: divisible by "),
        map_res(digit1, str::parse),
        tag("\n"),
    )(rem)?;

    let (rem, target_true) = delimited(
        tag("    If true: throw to monkey "),
        map_res(digit1, str::parse),
        tag("\n"),
    )(rem)?;
    let (rem, target_false) = preceded(
        tag("    If false: throw to monkey "),
        map_res(digit1, str::parse),
    )(rem)?; // don't eat the new line

    let monkey = Monkey {
        num,
        items: VecDeque::from_iter(items),
        new: op,
        test_divisor: divisor,
        target_monkeys: (target_false, target_true),
    };
    Ok((rem, monkey))
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Monkey>> {
    separated_list1(pair(line_ending, line_ending), parse_monkey)(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 10605);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 2713310158);
    }
}
