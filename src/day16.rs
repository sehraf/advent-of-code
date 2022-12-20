use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array3, ArrayBase, Dim, OwnedRepr};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
};

#[derive(Debug, Clone)]
pub struct Pipe {
    name: String,
    flow_rate: i32,
    connections: Vec<String>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Pipe> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

fn do_it(
    valves: &Vec<(&str, u16, Vec<&str>)>,
) -> (ArrayBase<OwnedRepr<u16>, Dim<[usize; 3]>>, usize, usize) {
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for (name, flow2, connections) in valves.iter() {
        let i = lab2idx[name];
        flow[i] = *flow2;
        for w in connections.iter() {
            adj[i].push(lab2idx[w]);
        }
    }
    let aa = lab2idx["AA"];

    let mm = 1 << m; // m = number of valves with positive flow

    // dynamic programming [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    (opt, aa, mm)
}

#[aoc(day16, part1)]
pub fn part1(input: &[Pipe]) -> u16 {
    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for i in input {
        valves.push((
            i.name.as_str(),
            i.flow_rate as u16,
            i.connections.iter().map(|c| c.as_str()).collect(),
        ));
    }
    valves.sort_by(|a, b| b.1.cmp(&a.1));

    let (opt, aa, mm) = do_it(&valves);

    opt[(29, aa, mm - 1)]
}

#[aoc(day16, part2)]
pub fn part2(input: &[Pipe]) -> u16 {
    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for i in input {
        valves.push((
            i.name.as_str(),
            i.flow_rate as u16,
            i.connections.iter().map(|c| c.as_str()).collect(),
        ));
    }
    valves.sort_by(|a, b| b.1.cmp(&a.1));

    let (opt, aa, mm) = do_it(&valves);

    let mut best = 0;
    for x in 0..mm {
        for y in 0..x {
            if (x & y) == 0 {
                best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
            }
        }
    }
    best
}

fn parse_line(input: &str) -> nom::IResult<&str, Pipe> {
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
        |(name, flow_rate, tunnels)| Pipe {
            name: name.to_owned(),
            flow_rate,
            connections: tunnels.iter().map(|t| t.to_string()).collect(),
        },
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Pipe>> {
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
