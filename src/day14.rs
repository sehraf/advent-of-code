use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

type Pos = (i32, i32);
type Stone = Vec<Pos>;

fn add_pos(a: &Pos, b: &Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

const GROUND_OFFSET: i32 = 2;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (HashSet<Pos>, i32) {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty());

    let mut map = HashSet::new();

    // lowest stone position
    let mut void = 0;

    // build cave
    for entry in ret {
        for s in entry.windows(2) {
            let start = s[0];
            let end = s[1];

            match (start, end) {
                // horizontal
                (start, end) if start.0 == end.0 => {
                    let y1 = start.1.min(end.1);
                    let y2 = start.1.max(end.1);
                    let x = start.0;
                    for y in y1..=y2 {
                        map.insert((x, y));
                    }
                    void = void.max(y2);
                }

                // vertical
                (start, end) if start.1 == end.1 => {
                    let x1 = start.0.min(end.0);
                    let x2 = start.0.max(end.0);
                    let y = start.1;
                    for x in x1..=x2 {
                        map.insert((x, y));
                    }
                    void = void.max(y);
                }

                _ => unreachable!(),
            }
        }
    }

    // build ground
    let ground = void + GROUND_OFFSET;
    // this is the largest possible range
    for x in 500 - ground - 1..=500 + ground + 1 {
        map.insert((x, ground));
    }

    #[cfg(test)]
    pretty_print(&map);

    (map, void)
}

macro_rules! drop {
    ($map:expr, $void:expr, $sand:expr => $dir:expr) => {
        let new_pos = add_pos(&$sand, &$dir);
        // empty space found?
        if !$map.contains(&new_pos) {
            $sand = new_pos;
            // restart "drop loop"
            continue;
        }
    };
    ($map:expr, $void:expr, $sand:expr) => {
        // down - or up on the y achsis
        drop!($map, $void, $sand => (0,1));

        // diag left
        drop!($map, $void, $sand => (-1,1));

        // diag right
        drop!($map, $void, $sand => (1,1));
    }
}

fn drop_sand(map: &HashSet<Pos>, void: i32, part_2: bool) -> Option<Pos> {
    let mut sand = (500, 0);

    if part_2 {
        // part 2: abort condition = stacked up to spawn of "sand"
        if map.contains(&sand) {
            return None;
        }
    }

    loop {
        if !part_2 {
            // part 1: abort condition = reached void
            if sand.1 >= void {
                return None;
            }
        }

        // will "continue" when dropped successfully
        drop!(map, void, sand);

        // no "continue" = no space found -> settled
        break;
    }

    Some(sand)
}

#[allow(dead_code)]
fn pretty_print(map: &HashSet<Pos>) {
    for y in 0..12 {
        for x in 490..510 {
            if x == 500 && y == 0 {
                print!("S");
            } else {
                match map.get(&(x, y)) {
                    None => print!("."),
                    Some(_) => print!("#"),
                }
            }
        }
        println!("")
    }
    println!("")
}

fn do_it(input: &(HashSet<Pos>, i32), part_2: bool) -> usize {
    let mut map = input.0.to_owned();
    let void = input.1;
    let mut counter = 0;

    // drop sand
    while let Some(pos) = drop_sand(&map, void, part_2) {
        // settled
        assert!(map.insert(pos));

        #[cfg(test)]
        {
            println!("counter: {counter}");
            pretty_print(&map);
        }

        counter += 1;
    }

    counter
}

#[aoc(day14, part1)]
pub fn part1(input: &(HashSet<Pos>, i32)) -> usize {
    do_it(input, false)
}

#[aoc(day14, part2)]
pub fn part2(input: &(HashSet<Pos>, i32)) -> usize {
    do_it(input, true)
}

fn parse_pos(input: &str) -> nom::IResult<&str, Pos> {
    separated_pair(
        map_res(digit1, str::parse),
        tag(","),
        map_res(digit1, str::parse),
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Stone>> {
    separated_list1(line_ending, separated_list1(tag(" -> "), parse_pos))(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 24);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 93);
    }
}
