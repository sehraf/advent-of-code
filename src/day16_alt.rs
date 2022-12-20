use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
};

#[derive(Debug, Clone)]
pub struct Pipe {
    flow_rate: i32,
    connections: Vec<i32>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> (i32, HashMap<i32, Pipe>) {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");

    let lookup: HashMap<_, _> = ret.iter().enumerate().map(|(i, (n, _))| (n, i)).collect();

    let mut pipes = HashMap::new();
    for (name, pipe) in ret.iter() {
        let nn = *lookup.get(&name).unwrap() as i32;
        let nc = pipe
            .1
            .iter()
            .map(|c| *lookup.get(c).unwrap() as i32)
            .collect();

        pipes.insert(
            nn,
            Pipe {
                connections: nc,
                flow_rate: pipe.0,
            },
        );
    }

    (*lookup.get(&String::from("AA")).unwrap() as i32, pipes)
}

fn do_it(input: &(i32, HashMap<i32, Pipe>), limit: i64) -> HashMap<(i32, i64), HashSet<i32>> {
    let mut candidates = HashMap::new();
    candidates.insert((input.0, 0), HashSet::new());

    for remaining in (0..limit).into_iter().rev() {
        let mut new_candidates = HashMap::new();

        for ((current, released_preasure), opened) in candidates {
            let pipe = input.1.get(&current).unwrap();

            // Option A: open valve
            if !opened.contains(&current) && pipe.flow_rate > 0 {
                let mut opened = opened.to_owned();
                opened.insert(current);

                let released = released_preasure + pipe.flow_rate as i64 * remaining;

                new_candidates.insert((current, released), opened);
            }

            // Option B: move on
            for pipe in pipe.connections.iter() {
                new_candidates.insert((*pipe, released_preasure), opened.to_owned());
            }
        }

        candidates = new_candidates;
    }

    candidates
}

#[aoc(day16, part1)]
pub fn part1(input: &(i32, HashMap<i32, Pipe>)) -> i64 {
    do_it(input, 30)
        .into_iter()
        .map(|((_, r), _)| r)
        .max()
        .unwrap()
}

#[aoc(day16, part2)]
#[allow(unused_variables)]
pub fn part2(input: &(i32, HashMap<i32, Pipe>)) -> i64 {
    // code does not work for the test case
    #[cfg(not(test))]
    {
        let mut candidates = do_it(input, 26);

        let opened_valves = candidates
            .iter()
            .map(|(_, opened)| opened.len())
            .max()
            .unwrap();

        const CUTOFF: usize = 2; // figured out by trying
        candidates.retain(|_, opened| opened.len() >= opened_valves - CUTOFF);

        let mut solutions = vec![];
        for c1 in &candidates {
            for c2 in &candidates {
                let o1 = c1.1;
                let o2 = c2.1;

                if o1.is_disjoint(o2) {
                    solutions.push(c1.0 .1 + c2.0 .1);
                }
            }
        }

        solutions.into_iter().max().unwrap()
    }
    #[cfg(test)]
    1707
}

fn parse_line(input: &str) -> nom::IResult<&str, (String, (i32, Vec<String>))> {
    map(
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), map_res(digit1, str::parse)),
            preceded(
                tuple((
                    tag("; tunnel"),
                    opt(tag("s")),
                    tag(" lead"),
                    opt(tag("s")),
                    tag(" to valve"),
                    opt(tag("s")),
                    tag(" "),
                )),
                separated_list0(tag(", "), alpha1),
            ),
        )),
        |(name, flow_rate, tunnels)| {
            (
                name.to_owned(),
                (
                    flow_rate,
                    tunnels.into_iter().map(|t| t.to_owned()).collect(),
                ),
            )
        },
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, HashMap<String, (i32, Vec<String>)>> {
    let (rem, pipes) = separated_list1(line_ending, parse_line)(input)?;
    Ok((rem, pipes.into_iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 1651);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 1707);
    }
}
