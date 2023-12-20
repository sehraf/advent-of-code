use std::collections::{hash_map::Entry, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use tracing::{info, warn};

#[derive(Debug, PartialEq, Clone)]

pub enum Signal {
    HighPulse,
    LowPulse,
}

pub trait HandleSignal {
    fn add_receiver(&mut self, from: &N);
    fn handle(&mut self, from: &N, signal: Signal) -> Option<(Signal, Vec<N>)>;
}

#[derive(Debug, Clone)]
pub struct FlipFlop {
    state: bool,
    connected_to: Vec<N>,
}

impl HandleSignal for FlipFlop {
    fn add_receiver(&mut self, _from: &N) {}

    fn handle(&mut self, _from: &N, signal: Signal) -> Option<(Signal, Vec<N>)> {
        match signal {
            Signal::HighPulse => None,
            Signal::LowPulse => {
                self.state = !self.state;
                let s = match self.state {
                    true => Signal::HighPulse,
                    false => Signal::LowPulse,
                };
                Some((s, self.connected_to.to_owned()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Broadcaster {
    connected_to: Vec<N>,
}

impl HandleSignal for Broadcaster {
    fn add_receiver(&mut self, _from: &N) {}

    fn handle(&mut self, _from: &N, signal: Signal) -> Option<(Signal, Vec<N>)> {
        Some((signal, self.connected_to.to_owned()))
    }
}

#[derive(Debug, Clone)]
pub struct Conjunction {
    pub inputs: FxHashMap<N, Signal>,
    connected_to: Vec<N>,
}

impl HandleSignal for Conjunction {
    fn add_receiver(&mut self, from: &N) {
        assert_eq!(self.inputs.insert(*from, Signal::LowPulse), None);
    }

    fn handle(&mut self, from: &N, signal: Signal) -> Option<(Signal, Vec<N>)> {
        *self.inputs.get_mut(from).unwrap() = signal;
        if self.inputs.values().all(|s| *s == Signal::HighPulse) {
            Some((Signal::LowPulse, self.connected_to.to_owned()))
        } else {
            Some((Signal::HighPulse, self.connected_to.to_owned()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl HandleSignal for Node {
    fn add_receiver(&mut self, from: &N) {
        match self {
            Node::FlipFlop(f) => f.add_receiver(from),
            Node::Conjunction(c) => c.add_receiver(from),
            Node::Broadcaster(b) => b.add_receiver(from),
        }
    }

    fn handle(&mut self, from: &N, signal: Signal) -> Option<(Signal, Vec<N>)> {
        match self {
            Node::FlipFlop(f) => f.handle(from, signal),
            Node::Conjunction(c) => c.handle(from, signal),
            Node::Broadcaster(b) => b.handle(from, signal),
        }
    }
}

type N = u8;
type T = FxHashMap<N, Node>;

#[aoc_generator(day20)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> (T, Vec<N>) {
    let mut mapping = FxHashMap::default();
    mapping.insert(String::from("button"), 0);
    mapping.insert(String::from("broadcaster"), 1);
    mapping.insert(String::from("rx"), 2);
    let mut last_id = 10;

    let mut get_id = |s: String| -> N {
        if let Some(id) = mapping.get(&s) {
            return *id;
        }
        mapping.insert(s, last_id);
        last_id += 1;
        last_id - 1
    };

    let mut nodes: T = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("->").unwrap();
            let a = a.trim();

            let connected_to = b.split(',').map(|s| get_id(s.trim().to_string())).collect();
            let node = match a {
                "broadcaster" => Node::Broadcaster(Broadcaster { connected_to }),
                flip if flip.starts_with("%") => Node::FlipFlop(FlipFlop {
                    connected_to,
                    state: false,
                }),
                con if con.starts_with("&") => Node::Conjunction(Conjunction {
                    connected_to,
                    inputs: FxHashMap::default(),
                }),
                n => unreachable!("reached unknown node {n}"),
            };

            let a = match a {
                flip if flip.starts_with("%") => &flip[1..],
                con if con.starts_with("&") => &con[1..],
                x => x,
            }
            .to_string();

            (get_id(a.to_string()), node)
        })
        .collect();

    // update conjunctions
    input.lines().for_each(|line| {
        let (from, to) = line.split_once("->").unwrap();
        let from: &str = from.trim();

        let from: &str = match from {
            flip if flip.starts_with("%") => &flip[1..],
            con if con.starts_with("&") => &con[1..],
            x => x,
        };
        let from = get_id(from.to_string());

        to.split(',')
            .map(|s| get_id(s.trim().to_string()))
            .for_each(|n| match nodes.entry(n) {
                Entry::Occupied(mut occ) => occ.get_mut().add_receiver(&from),
                Entry::Vacant(_vac) => {
                    // thanks AoC for having loose ends
                    // rx goes here
                }
            })
    });

    // part 2, get loop IDs
    let loops = if !cfg!(test) {
        let rx = &input
            .lines()
            .find(|line| line.ends_with("rx"))
            .unwrap()
            .split_ascii_whitespace()
            .next()
            .unwrap()[1..];
        input
            .lines()
            .filter(|line| line.ends_with(rx))
            .map(|line| &line.split_ascii_whitespace().next().unwrap()[1..])
            .map(|id| get_id(id.to_string()))
            .collect()
    } else {
        vec![]
    };

    (nodes, loops)
}

#[aoc(day20, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &(T, Vec<N>)) -> u64 {
    let mut state = input.0.to_owned();
    let mut cnt_low = 0;
    let mut cnt_high = 0;

    for _i in 0..1000 {
        let start_signal = (Signal::LowPulse, 0, 1);
        let mut signals = VecDeque::from([start_signal]);

        while let Some((signal, from, receiver)) = signals.pop_front() {
            match signal {
                Signal::HighPulse => cnt_high += 1,
                Signal::LowPulse => cnt_low += 1,
            }

            if !state.contains_key(&receiver) {
                continue;
            }

            let node = state.get_mut(&receiver).unwrap();
            let recv = node.handle(&from, signal);

            match recv {
                None => {}
                Some(recv) => {
                    for r in recv.1 {
                        signals.push_back((recv.0.to_owned(), receiver.to_owned(), r))
                    }
                }
            }
        }
    }

    cnt_low * cnt_high
}

#[aoc(day20, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &(T, Vec<N>)) -> u64 {
    let mut state = input.0.to_owned();
    let mut loops: FxHashMap<_, _> = input.1.iter().map(|id| (*id, 0)).collect();

    for i in 1.. {
        let start_signal = (Signal::LowPulse, 0, 1);
        let mut signals = VecDeque::from([start_signal]);

        while let Some((signal, from, receiver)) = signals.pop_front() {
            if !state.contains_key(&receiver) {
                continue;
            }

            let node = state.get_mut(&receiver).unwrap();
            let recv = node.handle(&from, signal);

            match recv {
                None => {}
                Some(recv) => {
                    let signal = recv.0;
                    for r in recv.1 {
                        signals.push_back((signal.to_owned(), receiver.to_owned(), r));

                        // check loops
                        if signal == Signal::LowPulse && loops.get(&r) == Some(&0) {
                            loops.insert(r, i);
                            if loops.values().all(|&v| v > 0) {
                                let mut lcm = 1;
                                for mut c in loops.values().copied() {
                                    let d = lcm * c;
                                    while c != 0 {
                                        (lcm, c) = (c, lcm % c);
                                    }
                                    lcm = d / lcm;
                                }
                                return lcm;
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1};

    const INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    #[test_log::test]
    fn test11() {
        assert_eq!(part1(&input_generator(INPUT1)), 32000000);
    }

    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    #[test_log::test]
    fn test12() {
        assert_eq!(part1(&input_generator(INPUT2)), 11687500);
    }
}
