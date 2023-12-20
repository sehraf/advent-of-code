use std::collections::{hash_map::Entry, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use tracing::info;

#[derive(Debug, PartialEq, Clone)]

pub enum Signal {
    HighPulse,
    LowPulse,
}

pub trait HandleSignal {
    fn add_receiver(&mut self, from: &str);
    fn handle(&mut self, from: &str, signal: Signal) -> Option<(Signal, Vec<String>)>;
}

#[derive(Debug, Clone)]
pub struct FlipFlop {
    state: bool,
    connected_to: Vec<String>,
}

impl HandleSignal for FlipFlop {
    fn add_receiver(&mut self, _from: &str) {}

    fn handle(&mut self, _from: &str, signal: Signal) -> Option<(Signal, Vec<String>)> {
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
    connected_to: Vec<String>,
}

impl HandleSignal for Broadcaster {
    fn add_receiver(&mut self, _from: &str) {}

    fn handle(&mut self, _from: &str, signal: Signal) -> Option<(Signal, Vec<String>)> {
        Some((signal, self.connected_to.to_owned()))
    }
}

#[derive(Debug, Clone)]
pub struct Conjunction {
    inputs: FxHashMap<String, Signal>,
    connected_to: Vec<String>,
}

impl HandleSignal for Conjunction {
    fn add_receiver(&mut self, from: &str) {
        assert_eq!(self.inputs.insert(from.to_string(), Signal::LowPulse), None);
    }

    fn handle(&mut self, from: &str, signal: Signal) -> Option<(Signal, Vec<String>)> {
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
    fn add_receiver(&mut self, from: &str) {
        match self {
            Node::FlipFlop(f) => f.add_receiver(from),
            Node::Conjunction(c) => c.add_receiver(from),
            Node::Broadcaster(b) => b.add_receiver(from),
        }
    }

    fn handle(&mut self, from: &str, signal: Signal) -> Option<(Signal, Vec<String>)> {
        match self {
            Node::FlipFlop(f) => f.handle(from, signal),
            Node::Conjunction(c) => c.handle(from, signal),
            Node::Broadcaster(b) => b.handle(from, signal),
        }
    }
}

type T = FxHashMap<String, Node>;

#[aoc_generator(day20)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let mut nodes: T = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("->").unwrap();
            let a: &str = a.trim();

            let connected_to = b.split(',').map(|s| s.trim().to_string()).collect();
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

            (a, node)
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

        to.split(',')
            .map(|s| s.trim().to_string())
            .for_each(|n| match nodes.entry(n) {
                Entry::Occupied(mut occ) => occ.get_mut().add_receiver(from),
                Entry::Vacant(_vac) => {
                    // thanks AoC for having loose ends
                    // rx goes here
                }
            })
    });

    nodes
}

fn push_button(state: &mut T) -> (u64, u64) {
    let mut cnt_low = 0;
    let mut cnt_high = 0;

    let start_signal = (
        Signal::LowPulse,
        String::from("button"),
        String::from("broadcaster"),
    );
    let mut signals = VecDeque::from([start_signal]);

    while let Some((signal, from, receiver)) = signals.pop_front() {
        // info!("{from} -{signal:?}-> {receiver}");
        match signal {
            Signal::HighPulse => cnt_high += 1,
            Signal::LowPulse => cnt_low += 1,
        }

        // thanks AoC for having loose ends
        if !state.contains_key(&receiver) {
            println!("{from} -{signal:?}-> {receiver}");
            if receiver == "rx" && signal == Signal::LowPulse {
                // yes I'm an adult
                return (420, 69);
            }
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

    (cnt_low, cnt_high)
}

#[aoc(day20, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u64 {
    let mut state = input.to_owned();
    let mut cnt_low = 0;
    let mut cnt_high = 0;

    for _i in 0..1000 {
        let (l, h) = push_button(&mut state);
        cnt_low += l;
        cnt_high += h;
    }

    cnt_low * cnt_high
}

#[aoc(day20, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    let mut state = input.to_owned();

    for i in 1..u32::MAX {
        let (l, h) = push_button(&mut state);

        if l == 420 && h == 69 {
            return i;
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
        // 773115468 too low
        assert_eq!(part1(&input_generator(INPUT2)), 11687500);
    }
}
