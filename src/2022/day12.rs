use std::collections::{hash_map::Entry, HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
};

type Pos = (i32, i32);

fn add_pos(a: &Pos, b: &Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Debug)]
pub enum Height {
    H(i32),
    Start,
    End,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HashMap<Pos, Height> {
    let (rem, hm) = parse(input).unwrap();
    debug_assert!(rem.is_empty());
    hm
}

#[aoc(day12, part1)]
pub fn part1(input: &HashMap<Pos, Height>) -> u32 {
    let (start, _) = input
        .iter()
        .find(|&(_, val)| matches!(val, Height::Start))
        .unwrap();
    let (end, _) = input
        .iter()
        .find(|&(_, val)| matches!(val, Height::End))
        .unwrap();

    let mut score = HashMap::new();
    let mut candidates = VecDeque::new();

    // add start
    score.insert(*start, 0);
    candidates.push_back(*start);

    while let Some(candidate) = candidates.pop_front() {
        let s = score.get(&candidate).unwrap().to_owned(); // if we crash here, the code it broken and requires fixing

        if candidate == *end {
            continue;
        }

        let next = visit(input, &candidate);

        for n in next {
            match score.entry(n) {
                Entry::Vacant(v) => {
                    _ = {
                        v.insert(s + 1);
                        candidates.push_back(n);
                    }
                }
                Entry::Occupied(mut o) => {
                    if *o.get() > s + 1 {
                        o.insert(s + 1);
                        candidates.push_back(n);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    {
        for y in 0..5 {
            for x in 0..8 {
                match score.get(&(x, y)) {
                    Some(s) => print!("{s:>3}"),
                    None => print!("   "),
                }
            }
            println!("");
        }
    }
    // #[cfg(not(test))]
    // {
    //     for y in 0..40 {
    //         for x in 0..144 {
    //             match score.get(&(x, y)) {
    //                 Some(s) => print!("{s:>3}"),
    //                 None => print!("   "),
    //             }
    //         }
    //         println!("");
    //     }
    // }

    score
        .into_iter()
        .find(|(pos, _)| pos == end)
        .map(|(_, s)| s)
        .unwrap()
}

fn visit(map: &HashMap<Pos, Height>, pos: &Pos) -> Vec<Pos> {
    let at = map.get(pos).unwrap();

    let mut candidates = vec![];

    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let new_pos = add_pos(pos, &dir);

        if let Some(now) = map.get(&new_pos) {
            match (at, now) {
                (Height::H(h1), Height::H(h2)) if h1 + 1 >= *h2 => candidates.push(new_pos),

                (Height::Start, Height::H(h2)) if *h2 == 0 || *h2 == 1 => candidates.push(new_pos),
                (Height::H(h1), Height::End) if *h1 == 24 || *h1 == 25 => candidates.push(new_pos),

                _ => {}
            }
        }
    }

    candidates
}

#[aoc(day12, part2)]
pub fn part2(input: &HashMap<Pos, Height>) -> u32 {
    let (_start, _) = input
        .iter()
        .find(|&(_, val)| matches!(val, Height::Start))
        .unwrap();
    let (end, _) = input
        .iter()
        .find(|&(_, val)| matches!(val, Height::End))
        .unwrap();

    let mut score = HashMap::new();
    let mut candidates = VecDeque::new();

    // add start
    score.insert(*end, 0);
    candidates.push_back(*end);

    while let Some(candidate) = candidates.pop_front() {
        let s = score.get(&candidate).unwrap().to_owned();
        let h = input.get(&candidate).unwrap();

        if matches!(h, Height::H(0)) {
            return s;
        }

        let next = visit_reverse(input, &candidate);

        for n in next {
            match score.entry(n) {
                Entry::Vacant(v) => {
                    _ = {
                        v.insert(s + 1);
                        candidates.push_back(n);
                    }
                }
                Entry::Occupied(mut o) => {
                    if *o.get() > s + 1 {
                        o.insert(s + 1);
                        candidates.push_back(n);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    {
        for y in 0..5 {
            for x in 0..8 {
                match score.get(&(x, y)) {
                    Some(s) => print!("{s:>3}"),
                    None => print!("   "),
                }
            }
            println!("");
        }
    }
    #[cfg(not(test))]
    {
        for y in 0..40 {
            for x in 0..144 {
                match score.get(&(x, y)) {
                    Some(s) => print!("{s:>3}"),
                    None => print!("   "),
                }
            }
            println!("");
        }
    }

    // let starting_points = input
    //     .iter()
    //     .filter_map(|(pos, h)| match h {
    //         Height::Start | Height::H(0) => Some(*pos),
    //         _ => None,
    //     })
    //     .collect::<Vec<_>>();
    // score
    //     .into_iter()
    //     .filter(|(pos, _)| starting_points.contains(pos))
    //     .map(|(_, s)| s)
    //     .min()
    //     .unwrap()
        0
}

fn visit_reverse(map: &HashMap<Pos, Height>, pos: &Pos) -> Vec<Pos> {
    let at = map.get(pos).unwrap();

    let mut candidates = vec![];

    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let new_pos = add_pos(pos, &dir);

        if let Some(now) = map.get(&new_pos) {
            match (at, now) {
                (Height::H(h1), Height::H(h2)) if *h1 <= h2 + 1 => candidates.push(new_pos),

                (Height::H(h1), Height::Start) if *h1 == 1 => candidates.push(new_pos),
                (Height::End, Height::H(h2)) if *h2 == 24 || *h2 == 25 => candidates.push(new_pos),

                _ => {}
            }
        }
    }

    candidates
}

fn parse(input: &str) -> nom::IResult<&str, HashMap<Pos, Height>> {
    let (rem, heights) = separated_list1(
        line_ending,
        map(alpha1, |c: &str| {
            c.chars()
                .map(|c| match c {
                    'S' => Height::Start,
                    'E' => Height::End,
                    c @ 'a'..='z' => {
                        let h = c as u8 - b'a';
                        Height::H(h as i32)
                    }
                    c @ _ => unreachable!("found {c}"),
                })
                .collect::<Vec<_>>()
        }),
    )(input)?;

    let mut hm = HashMap::new();
    for (y, row) in heights.into_iter().enumerate() {
        for (x, h) in row.into_iter().enumerate() {
            hm.insert((x as i32, y as i32), h);
        }
    }

    Ok((rem, hm))
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 31);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 29);
    }
}
