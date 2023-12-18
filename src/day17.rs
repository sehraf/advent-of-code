use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use glam::IVec2;

type T = (FxHashMap<IVec2, u32>, IVec2);

#[aoc_generator(day17)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let y = input.lines().count() as i32;
    let x = input.lines().next().unwrap().len() as i32;

    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices().map(move |(x, c)| {
                    (
                        IVec2 {
                            x: x as i32,
                            y: y as i32,
                        },
                        c.to_digit(10).unwrap(),
                    )
                })
            })
            .collect(),
        IVec2 { x, y },
    )
}

#[tracing::instrument(skip(map))]
fn find_path(map: &FxHashMap<IVec2, u32>, dim: &IVec2, part2: bool) -> u32 {
    let pos_start = IVec2::ZERO;
    let pos_end = *dim - IVec2::ONE;

    // heat_loss, position, direction (we went to or come from, doesn't really matter)
    let mut candidates: BTreeSet<(u32, [i32; 2], [i32; 2])> = BTreeSet::default();
    // keep track of direction as well!!
    let mut seen: FxHashSet<(IVec2, IVec2)> = FxHashSet::default();

    candidates.insert((0, pos_start.to_array(), [0, 0]));
    while let Some((heat_loss, pos, dir)) = candidates.pop_first() {
        let pos: IVec2 = pos.into();
        let dir: IVec2 = dir.into();

        if pos == pos_end {
            return heat_loss;
        }
        if !seen.insert((pos, dir)) {
            continue;
        }

        let dir_options = match dir {
            IVec2::X | IVec2::NEG_X => [IVec2::Y, IVec2::NEG_Y],
            IVec2::Y | IVec2::NEG_Y => [IVec2::X, IVec2::NEG_X],

            // special case for the start
            IVec2::ZERO => [IVec2::X, IVec2::Y],

            _ => unreachable!(),
        };
        for dir_new in dir_options {
            let mut hl = heat_loss;
            let mut new_pos = pos;

            if part2 {
                const MIN_STEP: i32 = 4;
                const MAX_STEP: i32 = 10;
                // start at `1` and go to (inclusive) `10`
                for step in 1..=MAX_STEP {
                    new_pos += dir_new;

                    if let Some(tile) = map.get(&new_pos) {
                        hl += tile;
                        if step >= MIN_STEP {
                            candidates.insert((hl, new_pos.to_array(), dir_new.to_array()));
                        }
                    } else {
                        break;
                    }
                }
            } else {
                for _ in 0..3 {
                    new_pos += dir_new;

                    if let Some(tile) = map.get(&new_pos) {
                        hl += tile;
                        candidates.insert((hl, new_pos.to_array(), dir_new.to_array()));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day17, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    let map = &input.0;
    let dim = input.1;

    find_path(map, &dim, false)
}

#[aoc(day17, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    let map = &input.0;
    let dim = input.1;

    find_path(map, &dim, true)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 102);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 94);
    }
}
