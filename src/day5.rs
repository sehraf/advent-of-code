use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, tuple},
};

#[derive(Debug, Clone)]
pub struct Move {
    count: usize,
    src: usize,
    dst: usize,
}
#[derive(Debug, Clone)]
pub struct State {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl State {
    pub fn get_tops(&self) -> String {
        self.stacks
            .as_slice()
            .iter()
            .map(|stack| match stack.front() {
                Some(a) => a.to_string(),
                None => String::new(),
            })
            .reduce(|acc, c| acc + &c)
            .unwrap()
    }

    pub fn run_moves(&mut self, is_9001: bool) {
        let moves = &self.moves;
        let stacks = &mut self.stacks;
        for Move { count, src, dst } in moves {
            // println!("moving {} from {} to {}", mv.count, mv.src, mv.dst);

            if is_9001 {
                let tmp = stacks[src - 1].drain(0..*count).rev().collect::<Vec<_>>();
                for x in tmp {
                    stacks[dst - 1].push_front(x)
                }
            } else {
                for _ in 0..*count {
                    match stacks[src - 1].pop_front() {
                        Some(a) => stacks[dst - 1].push_front(a),
                        None => (),
                    }
                }
            }

            // println!("{stacks:?}");
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> State {
    let (_, s) = parse(input).expect("failed to parse input");
    s
}

#[aoc(day5, part1)]
pub fn part1(input: &State) -> String {
    // println!("{input:?}");

    let mut state = input.clone();
    state.run_moves(false);
    state.get_tops()
}

#[aoc(day5, part2)]
pub fn part2(input: &State) -> String {
    let mut state = input.clone();
    state.run_moves(true);
    state.get_tops()
}

fn parse_stack_crate(input: &str) -> nom::IResult<&str, char> {
    delimited(char('['), anychar, char(']'))(input)
}

const EMPTY_STACK: &str = "   ";
fn parse_stack_empty(input: &str) -> nom::IResult<&str, char> {
    nom::combinator::map(tag(EMPTY_STACK), |_| ' ')(input)
}

fn parse_stack(input: &str) -> nom::IResult<&str, Vec<char>> {
    separated_list1(char(' '), alt((parse_stack_crate, parse_stack_empty)))(input)
}

fn parse_moves(line: &str) -> nom::IResult<&str, Move> {
    nom::combinator::map(
        tuple((
            tag("move "),
            map_res(digit1, str::parse),
            tag(" from "),
            map_res(digit1, str::parse),
            tag(" to "),
            map_res(digit1, str::parse),
        )),
        |(_, count, _, src, _, dst)| Move { count, src, dst },
    )(line)
}

fn parse(input: &str) -> nom::IResult<&str, State> {
    let halves = input.split_once("\n\n").unwrap();
    let (rem, stacks_horizontal) = separated_list1(line_ending, parse_stack)(halves.0)?;

    dbg!(&rem);
    // println!("{stacks_horizontal:#?}");

    let num = (rem.chars().count() + 1) / 4;

    // flip stack to vertical layout
    let mut stacks: Vec<VecDeque<char>> = vec![];
    for _ in 0..num {
        stacks.push(VecDeque::new());
    }

    for stack_h in stacks_horizontal.into_iter() {
        // println!("{stack_h:?}");
        for (pos, c) in stack_h.into_iter().enumerate() {
            let pos = pos % num;
            // println!("{pos}: '{c}'");

            match c {
                'A'..='Z' => stacks[pos].push_back(c),
                ' ' => (),
                _ => panic!("unexpected character on stack"),
            }
        }
    }

    let mut moves = vec![];
    for line in halves.1.lines() {
        let (_, mv) = parse_moves(line)?;
        moves.push(mv);
    }

    Ok((rem, State { stacks, moves }))
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), "CMZ");
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), "MCD");
    }
}
