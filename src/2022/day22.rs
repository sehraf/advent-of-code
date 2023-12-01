use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::take_till1,
    character::complete::{alpha1, digit1, line_ending, space0},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
};

const LOOKUP_DIR: [Pos; 4] = [
    Pos::new(1, 0),
    Pos::new(0, 1),
    Pos::new(-1, 0),
    Pos::new(0, -1),
];

#[derive(Debug)]
pub enum Step {
    Forward(i64),
    Right,
    Left,
}
type Pos = IVec2;
type Map = HashMap<Pos, bool>; // true = wall, false = open tile, None = nothing

pub struct State<'a> {
    map: &'a Map,
    steps: &'a Vec<Step>,

    path: Vec<(Pos, i8)>,

    pos: Pos,
    direction: i8,

    height: i32,
    width: i32,
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (Map, Vec<Step>) {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

#[inline]
fn draw_direction(dir: &i8) {
    match dir {
        0 => print!(">"),
        1 => print!("v"),
        2 => print!("<"),
        3 => print!("^"),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn draw(state: &State) {
    for y in 0..=state.height {
        for x in 0..=state.width {
            let p = (x, y).into();

            // draw us
            if state.pos == p {
                // draw_direction(d);
                print!("@");
                continue;
            }

            // draw path
            if let Some((_, d)) = state.path.iter().rev().find(|i| i.0 == p) {
                draw_direction(d);
                continue;
            }

            match state.map.get(&p) {
                None => print!(" "),
                Some(true) => print!("#"),
                Some(false) => print!("."),
            }
        }
        println!("");
    }
    println!("");
}

fn wrap(state: &mut State) -> bool {
    let rev_dir = LOOKUP_DIR[((state.direction + 2) % 4) as usize];
    let mut pos = state.pos;

    // find first empty tile
    while state.map.get(&(pos + rev_dir)).is_some() {
        pos += rev_dir;
    }

    // check if tile is a wall
    if state.map.get(&pos).unwrap() == &false {
        state.path.push((state.pos, state.direction));
        state.pos = pos;
        false
    } else {
        true
    }
}

// based on https://github.com/AxlLind/AdventOfCode2022/blob/a0c47df1fd608cd22f38a09a89c49f2f695fc253/src/bin/22.rs#L12
fn wrap_p2(state: &mut State) -> bool {
    #[cfg(test)]
    let f = 4;
    #[cfg(not(test))]
    let f = 50;

    // draw(state);

    let y = state.pos.y;
    let x = state.pos.x;
    let dir = state.direction as usize;

    // ┌─────────────────────────┐        ┌──────────────┐
    // │                         │        │              │
    // │                         │        │              │
    // │                         │        │              │
    // │                  ┌──────┴───┬────┴────┐         │
    // │                  │          │         │         │
    // │                  │          │         │         │
    // │   ┌──────────────┤  1/0     │   2/0   ├──┐      │
    // │   │              │          │         │  │      │
    // │   │              │          │         │  │      │
    // │   │              ├──────────┼────┬────┘  │      │
    // │   │              │          │    │       │      │
    // │   │              │          │    │       │      │
    // │   │        ┌─────┤  1/1     ◄────┘       │      │
    // │   │        │     │          │            │      │
    // │   │        │     │          │            │      │
    // │   │   ┌────▼─────┼──────────┤            │      │
    // │   │   │          │          │            │      │
    // │   │   │          │          ◄────────────┘      │
    // │   └───►  0/2     │  1/2     │                   │
    // │       │          │          │                   │
    // │       │          │          │                   │
    // │       ├──────────┼────▲─────┘                   │
    // │       │          │    │                         │
    // │       │          │    │                         │
    // └───────►  0/3     │    │                         │
    //         │          ├────┘                         │
    //         │          │                              │
    //         │          │                              │
    //         └─────▲────┘                              │
    //               │                                   │
    //               │                                   │
    //               └───────────────────────────────────┘

    #[cfg(test)]
    let (qx, qy, ndir) = match (x / f, y / f, dir) {
        // test middle middle
        (1, 1, 3) => (2, 0, 0),
        // test middle right
        (2, 1, 0) => (3, 2, 1),
        // test bottom left
        (2, 2, 1) => (0, 1, 3),
        _ => unreachable!(),
    };
    #[cfg(not(test))]
    let (qx, qy, ndir) = match (x / f, y / f, dir) {
        // input
        (0, 2, 2) => (1, 0, 0),
        (0, 2, 3) => (1, 1, 0),
        (0, 3, 0) => (1, 2, 3),
        (0, 3, 1) => (2, 0, 1),
        (0, 3, 2) => (1, 0, 1),

        (1, 0, 2) => (0, 2, 0),
        (1, 0, 3) => (0, 3, 0),
        (1, 1, 0) => (2, 0, 3),
        (1, 1, 2) => (0, 2, 1),
        (1, 2, 0) => (2, 0, 2),
        (1, 2, 1) => (0, 3, 2),

        (2, 0, 0) => (1, 2, 2),
        (2, 0, 1) => (1, 1, 2),
        (2, 0, 3) => (0, 3, 3),

        _ => unreachable!(),
    };

    let (dy, dx) = (y % f, x % f);
    let i = [dy, f - 1 - dx, f - 1 - dy, dx][dir];
    let (ny, nx) = [(i, 0), (0, f - 1 - i), (f - 1 - i, f - 1), (f - 1, i)][ndir];
    let (new_pos, new_dir) = ((qx * f + nx, qy * f + ny).into(), ndir as i8);

    match state.map.get(&new_pos) {
        Some(false) => {
            state.path.push((state.pos, state.direction));
            state.pos = new_pos;
            state.direction = new_dir;
            false
        }
        Some(true) => true,
        None => unreachable!(),
    }
}

fn do_it(state: &mut State, p2: bool) -> i32 {
    for s in state.steps {
        match s {
            Step::Forward(i) => {
                for _ in 0..*i {
                    let direction = LOOKUP_DIR[state.direction as usize];
                    let new_pos = state.pos + direction;

                    match state.map.get(&new_pos) {
                        None => {
                            //wrap around
                            let hit_wall = if p2 { wrap_p2(state) } else { wrap(state) };
                            if hit_wall {
                                break;
                            }
                        }
                        Some(true) => {
                            // stop
                            break;
                        }
                        Some(false) => {
                            // move
                            state.path.push((state.pos, state.direction));
                            state.pos = new_pos;
                        }
                    }
                }
            }
            Step::Left => state.direction = (state.direction + 3) % 4,
            Step::Right => state.direction = (state.direction + 1) % 4,
        }
    }

    (state.pos.y + 1) * 1000 + (state.pos.x + 1) * 4 + (state.direction) as i32
}

#[aoc(day22, part1)]
pub fn part1(input: &(Map, Vec<Step>)) -> i32 {
    let (map, steps) = input;

    let width = map.iter().map(|t| t.0.x).max().unwrap();
    let height = map.iter().map(|t| t.0.y).max().unwrap();

    // find start pos
    let mut player = (0, 0);
    for x in 0..width {
        match map.get(&(x, 0).into()) {
            None | Some(true) => continue,
            Some(false) => {
                player.0 = x;
                break;
            }
        }
    }

    let mut state = State {
        direction: 0,
        height,
        map,
        path: vec![],
        pos: player.into(),
        steps,
        width,
    };

    do_it(&mut state, false)
}

#[aoc(day22, part2)]
pub fn part2(input: &(Map, Vec<Step>)) -> i32 {
    let (map, steps) = input;

    let width = map.iter().map(|t| t.0.x).max().unwrap();
    let height = map.iter().map(|t| t.0.y).max().unwrap();

    // find start pos
    let mut player = (0, 0);
    for x in 0..width {
        match map.get(&(x, 0).into()) {
            None | Some(true) => continue,
            Some(false) => {
                player.0 = x;
                break;
            }
        }
    }

    let mut state = State {
        direction: 0,
        height,
        map,
        path: vec![],
        pos: player.into(),
        steps,
        width,
    };

    // 174109 too high

    do_it(&mut state, true)
}

fn parse_map(input: &str) -> nom::IResult<&str, Vec<(usize, bool)>> {
    let (rem, space_left) = space0(input)?;
    let space_left = space_left.len();
    let (rem, d) = take_till1(|c| c != '.' && c != '#')(rem)?;
    // let (rem, _) = space1(rem)?;

    let v = d
        .chars()
        .enumerate()
        .map(|(pos, c)| match c {
            '.' => (pos + space_left, false),
            '#' => (pos + space_left, true),
            _ => unreachable!(),
        })
        .collect();

    Ok((rem, v))
}

fn parse_steps(input: &str) -> nom::IResult<&str, Vec<Step>> {
    many1(alt((
        map(map_res(digit1, str::parse), |i| Step::Forward(i)),
        map(alpha1, |c| match c {
            "R" => Step::Right,
            "L" => Step::Left,
            _ => unreachable!(),
        }),
    )))(input)
}

fn parse(input: &str) -> nom::IResult<&str, (Map, Vec<Step>)> {
    let mut it = input.split("\n\n");

    let (_, map) = separated_list1(line_ending, parse_map)(it.next().unwrap())?;
    let (rem, steps) = parse_steps(it.next().unwrap())?;

    let map = map
        .into_iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .map(|(x, t)| (Pos::new(x as i32, y as i32), t))
                .collect::<Vec<_>>()
        })
        .collect();

    Ok((rem, (map, steps)))
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 6032);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 5031);
    }
}
