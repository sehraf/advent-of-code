use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{character::complete::anychar, combinator::map, multi::many1};

// Map width
const WIDTH: i64 = 7;

// new rock spawn offset
const SPWAN_LEFT: i64 = 2;
const SPAWN_BOTTOM: i64 = 3;

// clean map cutoff
const CUTOFF: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(i64, i64);

impl Pos {}

impl From<(i64, i64)> for Pos {
    fn from(value: (i64, i64)) -> Self {
        Pos(value.0, value.1)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1).into()
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Shape {
    Minus,
    Plus,
    Hook,
    Pole,
    Dot,
}

impl Shape {
    pub fn spawn(&self) -> Vec<Pos> {
        match self {
            Self::Dot => vec![(0, 0), (0, 1), (1, 0), (1, 1)]
                .into_iter()
                .map_into()
                .collect(),
            Self::Hook => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
                .into_iter()
                .map_into()
                .collect(),
            Self::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)]
                .into_iter()
                .map_into()
                .collect(),
            Self::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
                .into_iter()
                .map_into()
                .collect(),
            Self::Pole => vec![(0, 0), (0, 1), (0, 2), (0, 3)]
                .into_iter()
                .map_into()
                .collect(),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Dot => Self::Minus,
            Self::Hook => Self::Pole,
            Self::Minus => Self::Plus,
            Self::Plus => Self::Hook,
            Self::Pole => Self::Dot,
        }
    }
}

pub struct State<'a> {
    next_spawn: Shape,
    next_is_down: bool,

    map: BTreeMap<Pos, bool>,
    active: Vec<Pos>,

    max_hight: i64,

    directions: &'a [Direction],
    next_direction: usize,

    rocks_dropped: i64,
}

fn draw(state: &State) {
    for y in (-5..6).rev() {
        print!("|");
        for x in 0..WIDTH {
            let pos = (x, y + state.max_hight).into();
            let active = state.active.contains(&pos);
            match (state.map.get(&pos), active) {
                (None, false) => print!("."),
                (None, true) => print!("@"),
                (Some(false), false) => print!("."),
                (Some(false), true) => print!("@"),
                (Some(true), false) => print!("#"),
                (Some(true), true) => unreachable!(),
            }
        }
        println!("|");
    }
    println!("");
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    let (rem, ret) = parse(input.lines().next().unwrap()).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

fn valid_move(state: &State, shape: &Vec<Pos>) -> bool {
    // valid move?
    // let hit_wall_left = shape.iter().any(|p| p.0 < 0);
    // let hit_wall_right = shape.iter().any(|p| p.0 >= WIDTH);
    let hit_smt = shape.iter().any(|p| p.0 < 0 || p.0 >= WIDTH || p.1 <= 0);
    let hit_existing = shape.iter().any(|p| match state.map.get(p) {
        Some(true) => true,
        _ => false,
    });

    !hit_existing && !hit_smt
}

fn round(state: &mut State) {
    if state.active.is_empty() {
        // Spawn new
        let new = state.next_spawn.to_owned();
        state.next_spawn = state.next_spawn.next();
        let mut shape = new.spawn();

        // move to start position
        let offset: Pos = (SPWAN_LEFT, state.max_hight + 1 + SPAWN_BOTTOM).into();
        shape.iter_mut().for_each(|p| *p += offset.to_owned());

        // println!("spawning new {new:?} @ {offset:?}");

        state.active = shape;
        state.rocks_dropped += 1;
        draw(&state);
    } else {
        if state.next_is_down {
            let vector: Pos = (0, -1).into();
            let new_active: Vec<_> = state
                .active
                .iter()
                .map(|p| p.to_owned() + vector.to_owned())
                .collect();

            if valid_move(state, &new_active) {
                // println!("Moving {vector:?}");
                state.active = new_active;
            } else {
                // println!("Cannot move {vector:?}");

                let mut new_max_hight = state.max_hight;
                for p in state.active.drain(..) {
                    new_max_hight = new_max_hight.max(p.1);
                    state.map.insert(p, true);
                }
                state.max_hight = new_max_hight;
                // println!("hit ground, new max height: {}", state.max_hight);
            }
        } else {
            let dir = &state.directions[state.next_direction];
            state.next_direction = (state.next_direction + 1) % state.directions.len();

            let vector: Pos = match dir {
                Direction::Left => (-1, 0).into(),
                Direction::Right => (1, 0).into(),
            };
            let new_active: Vec<_> = state
                .active
                .iter()
                .map(|p| p.to_owned() + vector.to_owned())
                .collect();

            if valid_move(state, &new_active) {
                // println!("Moving {vector:?}");
                state.active = new_active;
            } else {
                // println!("Cannot move {dir:?}");
            }
        }

        // toggle new move action
        state.next_is_down ^= true;
    }

    state.map.retain(|p, _| p.1 + CUTOFF >= state.max_hight);
}

#[aoc(day17, part1)]
pub fn part1(input: &[Direction]) -> i64 {
    let mut state = State {
        active: vec![],
        directions: input,
        map: BTreeMap::new(),
        max_hight: 0,
        next_spawn: Shape::Minus,
        next_is_down: false,
        next_direction: 0,
        rocks_dropped: 0,
    };

    loop {
        round(&mut state);
        // draw(&state);

        if state.rocks_dropped >= 2023 {
            break;
        }
    }

    state.max_hight
}

#[aoc(day17, part2)]
pub fn part2(_input: &[Direction]) -> i64 {
    todo!()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Direction>> {
    many1(map(anychar, |c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!(),
    }))(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 3068);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 1514285714288);
    }
}
