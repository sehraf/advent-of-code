use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet;
use glam::IVec2;

type T = (FxHashSet<IVec2>, IVec2);

#[aoc_generator(day21)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let mut gardens = FxHashSet::default();
    let mut start = IVec2 { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    gardens.insert(IVec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                'S' => {
                    start = IVec2 {
                        x: x as i32,
                        y: y as i32,
                    };
                    gardens.insert(IVec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '#' => {}
                _ => unimplemented!(),
            };
        }
    }

    (gardens, start)
}

#[tracing::instrument(skip(gardens))]
pub fn walk(gardens: &FxHashSet<IVec2>, starting_point: IVec2, step_limit: i32) -> usize {
    let mut seen_states = FxHashSet::default();
    let mut next_steps = VecDeque::new();
    let mut destinations = FxHashSet::default();

    next_steps.push_back((starting_point, 0));
    let original_bound = IVec2 {
        x: gardens.iter().map(|c| c.x).max().unwrap(),
        y: gardens.iter().map(|c| c.y).max().unwrap(),
    };

    while let Some((pos, steps_taken)) = next_steps.pop_front() {
        if steps_taken == step_limit {
            destinations.insert(pos);
            continue;
        }
        if seen_states.contains(&(pos, steps_taken)) {
            continue;
        }
        seen_states.insert((pos, steps_taken));
        for dir in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
            let pos_new = pos + dir;
            if gardens.contains(&IVec2 {
                x: pos_new.x.rem_euclid(original_bound.x + 1),
                y: pos_new.y.rem_euclid(original_bound.y + 1),
            }) {
                next_steps.push_back((pos_new, steps_taken + 1));
            }
        }
    }
    destinations.len()
}

#[aoc(day21, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> usize {
    let gardens = &input.0;
    let starting_point = input.1;
    let step_limit = if cfg!(test) { 6 } else { 64 };

    walk(gardens, starting_point, step_limit)
}

fn extrapolate_last_value(values: &[usize]) -> usize {
    let differences = values[1..]
        .iter()
        .enumerate()
        .map(|(i, x)| x - values[i])
        .collect::<Vec<_>>();
    if differences.iter().all(|x| x == &0) {
        *values.iter().last().unwrap()
    } else {
        values.iter().last().unwrap() + extrapolate_last_value(&differences)
    }
}

#[aoc(day21, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> usize {
    let gardens = &input.0;
    let starting_point = input.1;

    let mut progression = Vec::new();
    for i in 0..=2 {
        progression.push(walk(gardens, starting_point, 65 + i * 131));
    }
    println!("{:?}", progression);
    while progression.len() < (26501365 - 65) / 131 {
        progression.push(extrapolate_last_value(&progression))
    }
    extrapolate_last_value(&progression)
}

// thanks https://github.com/wilkotom/Aoc2023/blob/main/day21/src/main.rs

#[cfg(test)]
mod tests {
    use super::{input_generator, part1};

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 16);
    }
}
