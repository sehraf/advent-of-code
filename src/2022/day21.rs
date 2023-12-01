use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

const ROOT_MONKEY: &str = "root";
const ME_MONKEY: &str = "humn";

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Product,
    Division,
}

impl<S> From<S> for Op
where
    S: ToString,
{
    fn from(value: S) -> Self {
        match value.to_string().as_str() {
            " + " => Op::Plus,
            " - " => Op::Minus,
            " * " => Op::Product,
            " / " => Op::Division,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Monkey {
    Num(i64),
    Op(String, Op, String),
    EqualOp(String, Op, String, i64),
}

impl From<(&str, &str, &str)> for Monkey {
    fn from(value: (&str, &str, &str)) -> Self {
        Monkey::Op(value.0.to_string(), value.1.into(), value.2.to_string())
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(String, Monkey)> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

macro_rules! math {
    ($left:expr, $op:expr, $right:expr) => {
        match $op {
            Op::Plus => $left + $right,
            Op::Minus => $left - $right,
            Op::Product => $left * $right,
            Op::Division => $left / $right,
        }
    };
    (R => $op:expr, $left:expr, $target:expr) => {
        match $op {
            Op::Plus => $target - $left,     // l+r=t <=> r=t-l
            Op::Minus => $left - $target,    // l-r=t <=> r=l-t
            Op::Product => $target / $left,  // l*r=t <=> r=t/l
            Op::Division => $left / $target, // l/r=t <=> r=l/t
        }
    };
    (L => $op:expr, $right:expr, $target:expr) => {
        match $op {
            Op::Plus => $target - $right,     // l+r=t <=> l=t-r
            Op::Minus => $target + $right,    // l-r=t <=> l=t+r
            Op::Product => $target / $right,  // l*r=t <=> l=t/r
            Op::Division => $target * $right, // l/r=t <=> l=t*r
        }
    };
}

fn resolve(map: &mut HashMap<String, Monkey>, part_2: bool) -> bool {
    // "downwards" math
    for (name, monkey) in map.iter().filter(|(_, m)| matches!(m, Monkey::Op(_, _, _))) {
        if part_2 && (name == ROOT_MONKEY || name == ME_MONKEY) {
            continue;
        }

        // can we resolve this monkey?
        if let Monkey::Op(l, o, r) = monkey {
            // println!("found candidate");
            match (map.get(l), map.get(r)) {
                (Some(Monkey::Num(l)), Some(Monkey::Num(r))) => {
                    let v = math!(l, o, r);
                    let m_new = Monkey::Num(v);
                    map.insert(name.to_owned(), m_new);
                    return true;
                }
                _ => {}
            }
        } else {
            unreachable!()
        }
    }

    if !part_2 {
        return false;
    }

    // check for root
    if let Some(Monkey::Op(l, _, r)) = map.get(ROOT_MONKEY) {
        match (map.get(l), map.get(r)) {
            (Some(Monkey::Num(i)), Some(Monkey::Op(l2, o2, r2))) => {
                // we can update right monkey

                // hello borrow checker 👋
                let i = i.to_owned();
                let r = r.to_owned();

                let m_new = Monkey::EqualOp(l2.to_owned(), o2.to_owned(), r2.to_owned(), i);
                map.insert(r, m_new);
                map.insert(ROOT_MONKEY.to_owned(), Monkey::Num(i));
            }
            (Some(Monkey::Op(l2, o2, r2)), Some(Monkey::Num(i))) => {
                // we can update left monkey

                // hello borrow checker 👋
                let i = i.to_owned();
                let l = l.to_owned();

                let m_new = Monkey::EqualOp(l2.to_owned(), o2.to_owned(), r2.to_owned(), i);
                map.insert(l, m_new);
                map.insert(ROOT_MONKEY.to_owned(), Monkey::Num(i));
            }
            _ => {
                dbg!(&map);
                println!("failed to update root");
            }
        }
    }

    // "upwards" math
    for (name, monkey) in map
        .iter()
        .filter(|(_, m)| matches!(m, Monkey::EqualOp(_, _, _, _)))
    {
        // can we resolve this monkey?
        if let Monkey::EqualOp(l, o, r, t) = monkey {
            match (map.get(l), map.get(r)) {
                // handle the "normal" case aka. just calculate the target value for the next monkey
                (Some(Monkey::Num(i)), Some(Monkey::Op(l2, o2, r2))) => {
                    // we can update right monkey

                    // hello borrow checker 👋
                    let r = r.to_owned();
                    let orig_monkey = name.to_owned();

                    let t = math!(R => o, i, t);
                    let m_new = Monkey::EqualOp(l2.to_owned(), o2.to_owned(), r2.to_owned(), t);
                    map.insert(r, m_new);
                    map.insert(orig_monkey.to_owned(), Monkey::Num(t));
                    return true;
                }
                (Some(Monkey::Op(l2, o2, r2)), Some(Monkey::Num(i))) => {
                    // we can update left monkey

                    // hello borrow checker 👋
                    let l = l.to_owned();
                    let orig_monkey = name.to_owned();

                    let t = math!(L => o, i, t);
                    let m_new = Monkey::EqualOp(l2.to_owned(), o2.to_owned(), r2.to_owned(), t);
                    map.insert(l, m_new);
                    map.insert(orig_monkey.to_owned(), Monkey::Num(t));
                    return true;
                }

                // handle the case where we are the next "monkey"
                (Some(Monkey::Num(i)), None) if r == ME_MONKEY => {
                    // found us!
                    let t = math!(R => o, i, t);
                    map.insert(ME_MONKEY.to_owned(), Monkey::Num(t));
                    // just end the loop!
                    return false;
                }
                (None, Some(Monkey::Num(i))) if l == ME_MONKEY => {
                    // found us!
                    let t = math!(L => o, i, t);
                    map.insert(ME_MONKEY.to_owned(), Monkey::Num(t));
                    // just end the loop!
                    return false;
                }
                _ => {}
            }
        }
    }

    false
}

#[aoc(day21, part1)]
pub fn part1(input: &[(String, Monkey)]) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = input
        .iter()
        .map(|(n, m)| (n.to_owned(), m.to_owned()))
        .collect();

    while resolve(&mut monkeys, false) {}

    let Some(Monkey::Num(x)) = monkeys.get(ROOT_MONKEY) else {unreachable!()};
    *x
}

#[aoc(day21, part2)]
pub fn part2(input: &[(String, Monkey)]) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = input
        .iter()
        .map(|(n, m)| (n.to_owned(), m.to_owned()))
        .collect();

    // this is important! (otherwise "resolve" uses the value of humn);
    monkeys.remove(ME_MONKEY);

    while resolve(&mut monkeys, true) {}

    let Some(Monkey::Num(x)) = monkeys.get(ME_MONKEY) else {unreachable!()};
    *x
}

fn parse_monkey(input: &str) -> nom::IResult<&str, (String, Monkey)> {
    map(
        separated_pair(
            alpha1,
            tag(": "),
            alt((
                map(map_res(digit1, str::parse), |i| Monkey::Num(i)),
                map(
                    tuple((
                        alpha1,
                        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
                        alpha1,
                    )),
                    |m| m.into(),
                ),
            )),
        ),
        |(n, m)| (n.to_string(), m),
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(String, Monkey)>> {
    separated_list1(line_ending, parse_monkey)(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 152);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 301);
    }
}
