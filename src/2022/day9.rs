use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{digit1, line_ending, one_of, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_move(&self) -> (i32, i32) {
        match self {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
        }
    }
}

pub struct Steps {
    dir: Direction,
    num: u32,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Steps> {
    let (rem, v) = parse(input).unwrap();
    assert!(rem.is_empty());
    v
}

type Pos = (i32, i32);

#[derive(Debug)]
pub struct State {
    knots: Vec<Pos>,
    visited: HashMap<Pos, bool>,
}

impl State {
    pub fn handle_steps(&mut self, steps: &[Steps]) -> &HashMap<Pos, bool> {
        let make_step = |mv: (i32, i32), head_and_tail: &mut [Pos]| {
            // move a
            head_and_tail[0].0 += mv.0;
            head_and_tail[0].1 += mv.1;

            debug_assert!(head_and_tail[0].0 >= 0);
            debug_assert!(head_and_tail[0].1 >= 0);

            // calculate move b
            let diff_x = head_and_tail[0].0 - head_and_tail[1].0;
            let diff_y = head_and_tail[0].1 - head_and_tail[1].1;

            debug_assert!(diff_x <= 2);
            debug_assert!(diff_y <= 2);

            let mv_b = match (diff_x, diff_y) {
                // valid states
                (0, 0)
                | (1, 0)
                | (-1, 0)
                | (0, 1)
                | (0, -1)
                | (1, 1)
                | (-1, 1)
                | (1, -1)
                | (-1, -1) => (0, 0),

                // horizontal
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),

                // vertical
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),

                // diagonal
                (x, y) if x != 0 && y != 0 => (x.signum(), y.signum()),

                // everything else can't happen
                _ => unreachable!(),
            };

            // skip
            if mv_b == (0, 0) {
                return;
            }

            head_and_tail[1].0 += mv_b.0;
            head_and_tail[1].1 += mv_b.1;
        };

        // go through all steps
        for step in steps {
            // get the move to perform
            let mv = step.dir.to_move();

            // repeat the step!
            for _ in 0..step.num {
                // only the head must perform the move
                let mut tmp_mv = mv;
                for i in 0..self.knots.len() - 1 {
                    make_step(tmp_mv, &mut self.knots[i..=i + 1]);
                    // after the first iteration: head was moved, not set tmp_mv to (0,0)
                    tmp_mv = (0, 0);
                }

                // mark last knot's position as visited
                _ = self.visited.insert(*self.knots.last().unwrap(), true);
            }

            #[cfg(test)]
            {
                // pretty print knot snake
                for x in 0..=5 {
                    for y in 0..=4 {
                        match self
                            .knots
                            .iter()
                            .enumerate()
                            .filter(|(_, &k)| k == (x, y))
                            .last()
                        {
                            Some((n, _)) => print!("{n}"),
                            None => print!("."),
                        }
                    }
                    println!("");
                }
                println!("");
            }
        }

        &self.visited
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &[Steps]) -> usize {
    let mut state = State {
        knots: vec![(0, 0), (0, 0)],
        visited: HashMap::new(),
    };
    _ = state.visited.insert((0, 0), true);
    _ = state.handle_steps(input);

    state.visited.into_iter().filter(|(_, v)| *v).count()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Steps]) -> usize {
    let mut state = State {
        knots: Vec::from([(0, 0); 10]),
        visited: HashMap::new(),
    };
    _ = state.visited.insert((0, 0), true);
    _ = state.handle_steps(input);

    #[cfg(test)]
    {
        // pretty print
        for x in 0..=5 {
            for y in 0..=4 {
                print!(
                    "{}",
                    match state.visited.get(&(x, y)) {
                        Some(true) => "#",
                        Some(false) | None => ".",
                    }
                )
            }
            println!("");
        }
        println!("");
    }

    state.visited.into_iter().filter(|(_, v)| *v).count()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Steps>> {
    map(
        separated_list1(line_ending, separated_pair(one_of("UDLR"), space1, digit1)),
        |a| {
            a.into_iter()
                .map(|(d, i)| {
                    // fix to hint the type of `i`
                    let i: &str = i;

                    let num = i.parse().unwrap();
                    let dir = match d {
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        'U' => Direction::Up,
                        _ => unreachable!(),
                    };
                    Steps { dir, num }
                })
                .collect()
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 1);
    }
}
