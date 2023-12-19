use std::{collections::hash_map::Entry, ops::RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Field {
    X,
    M,
    A,
    S,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            'x' => Field::X,
            'm' => Field::M,
            'a' => Field::A,
            's' => Field::S,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        value.chars().next().unwrap().into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Accept,
    Reject,
    Continue(String),
}
impl From<&str> for Action {
    fn from(value: &str) -> Self {
        assert!(value.len() <= 3);
        match value {
            "A" => Action::Accept,
            "R" => Action::Reject,
            n => Action::Continue(n.to_string()),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    valid: bool, // indicates last rule that always applies
    field: Field,
    value: u32,
    greater: bool,
    action: Action,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_field(&self, field: &Field) -> u32 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    fn added(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
pub struct Input {
    rules: FxHashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

type T = Input;

#[tracing::instrument]
fn parse_rule(rule: &str) -> Rule {
    if rule.contains(':') {
        let field = rule[0..1].into();
        let greater = match &rule[1..2] {
            ">" => true,
            "<" => false,
            _ => unreachable!(),
        };

        let (value, action) = rule[2..].split_once(':').unwrap();

        Rule {
            valid: true,
            field,
            action: action.into(),
            value: value.parse().unwrap(),
            greater,
        }
    } else {
        Rule {
            valid: false,
            action: rule.into(),
            value: 0,
            greater: false,
            field: Field::A, // doesn't matter
        }
    }
}

#[tracing::instrument]
fn parse_part(part: &str) -> Part {
    let mut it = part.split(',');
    let x = it.next().unwrap()[3..].parse().unwrap();
    let m = it.next().unwrap()[2..].parse().unwrap();
    let a = it.next().unwrap()[2..].parse().unwrap();
    let s = &it.next().unwrap()[2..];
    let s = s[..s.len() - 1].parse().unwrap();
    Part { x, m, a, s }
}

#[aoc_generator(day19)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let (rules, parts) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rules = rest[..rest.len() - 1].split(',').map(parse_rule).collect();
            (name.to_string(), rules)
        })
        .collect();

    let parts = parts.lines().map(parse_part).collect();

    Input { rules, parts }
}

#[aoc(day19, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    let start = "in";
    let rules = &input.rules;
    let parts = &input.parts;

    parts
        .iter()
        .filter(|part| {
            let mut state = start;
            loop {
                let rule = rules.get(state).unwrap();
                for r in rule {
                    let action = if r.valid {
                        let part_val = part.get_field(&r.field);
                        let applies = match r.greater {
                            true => part_val > r.value,
                            false => part_val < r.value,
                        };

                        if !applies {
                            continue;
                        }
                        &r.action
                    } else {
                        &r.action
                    };

                    match action {
                        Action::Accept => return true,
                        Action::Reject => return false,
                        Action::Continue(next) => {
                            state = &next;
                            break;
                        }
                    }
                }
            }
        })
        .map(|part| part.added())
        .sum()
}

#[aoc(day19, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u64 {
    let rules = &input.rules;

    let start = ("in", vec![]);
    let mut candidates = vec![start];
    let mut winner: Vec<Vec<Rule>> = vec![];

    // collect positive paths
    while let Some((state, history)) = candidates.pop() {
        let rule = rules.get(state).unwrap();
        // we need to keep track of not applied rules
        let mut anti_filter = vec![];

        for r in rule {
            // create current rule set
            let mut filter = history.to_owned();
            // append previous anti rules
            filter.append(&mut anti_filter.to_owned());

            // update filter if rule it valid
            if r.valid {
                // append current rule
                filter.push(r.to_owned());

                // convert greater/less into greater equal / less equal
                let value = if r.greater {
                    if r.value < 4000 {
                        r.value + 1
                    } else {
                        4000
                    }
                } else {
                    if r.value > 1 {
                        r.value - 1
                    } else {
                        1
                    }
                };
                // build anti rule
                let anti = Rule {
                    valid: true,
                    field: r.field,
                    value,
                    greater: !r.greater,
                    action: Action::Reject, // doesn't matter
                };
                anti_filter.push(anti);
            }

            match &r.action {
                Action::Reject => continue,
                Action::Accept => winner.push(filter),
                Action::Continue(next) => candidates.push((&next, filter)),
            }
        }
    }

    winner
        .into_iter()
        // condense XMAS parameter
        .map(|rules| {
            let mut filter: FxHashMap<Field, Vec<Rule>> = FxHashMap::default();
            rules
                .into_iter()
                .for_each(|rule| match filter.entry(rule.field) {
                    Entry::Occupied(mut occ) => occ.get_mut().push(rule),
                    Entry::Vacant(vac) => _ = vac.insert(vec![rule]),
                });
            filter
        })
        // create ranges
        .map(|filter| {
            let mut ranges: FxHashMap<Field, RangeInclusive<u32>> = FxHashMap::default();
            for (key, filter) in filter {
                let mut range = 1..=4000;
                for r in filter {
                    // check for valid range
                    // assert!(*range.start() < r.value && r.value < *range.end());
                    // apparently this is always the case

                    // create new range
                    match r.greater {
                        true => range = r.value + 1..=*range.end(),
                        false => range = *range.start()..=r.value - 1,
                    }
                }

                match ranges.entry(key) {
                    Entry::Occupied(_) => unreachable!(),
                    Entry::Vacant(vac) => _ = vac.insert(range),
                }
            }
            ranges
        })
        // count options
        .map(|mut ranges| {
            let mut sum = 1;

            sum *= ranges.entry(Field::X).or_insert(1..=4000).count();
            sum *= ranges.entry(Field::M).or_insert(1..=4000).count();
            sum *= ranges.entry(Field::A).or_insert(1..=4000).count();
            sum *= ranges.entry(Field::S).or_insert(1..=4000).count();

            sum as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 19114);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 167409079868000);
    }
}
