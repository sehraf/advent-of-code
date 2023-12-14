use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use glam::IVec2;
use itertools::Itertools;
use tracing::info;

// true == O
// false == #
// not found == .
type T = (FxHashMap<IVec2, bool>, IVec2);

#[aoc_generator(day14)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let y = input.lines().count() as i32;
    let x = input.lines().next().unwrap().len() as i32;

    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(|(x, c)| match c {
                        '.' => None,
                        'O' => Some((
                            IVec2 {
                                x: x as i32,
                                y: y as i32,
                            },
                            true,
                        )),
                        '#' => Some((
                            IVec2 {
                                x: x as i32,
                                y: y as i32,
                            },
                            false,
                        )),
                        _ => unreachable!(),
                    })
                    .collect::<FxHashMap<IVec2, bool>>()
            })
            .collect(),
        IVec2 { x, y },
    )
}

#[tracing::instrument(skip(input))]
fn tilt(input: &mut FxHashMap<IVec2, bool>, dim: &IVec2, dir: &IVec2) {
    // all keys in the correct order
    let keys: Vec<_> = {
        match dir {
            IVec2 { x, y } if *x == 1 && *y == 0 => {
                ((0..dim.x).rev()).cartesian_product(0..dim.y).collect()
            }
            IVec2 { x, y } if *x == 0 && *y == 1 => {
                ((0..dim.y).rev()).cartesian_product(0..dim.x).collect()
            }
            IVec2 { x, y } if *x == -1 && *y == 0 => {
                (0..=(dim.x - 1)).cartesian_product(0..dim.y).collect()
            }
            IVec2 { x, y } if *x == 0 && *y == -1 => {
                (0..=(dim.y - 1)).cartesian_product(0..dim.x).collect()
            }
            _ => unreachable!(),
        }
    };

    for key in keys {
        let pos = match dir {
            IVec2 { x, y } if *x == 1 && *y == 0 => IVec2 { x: key.0, y: key.1 },
            IVec2 { x, y } if *x == 0 && *y == 1 => IVec2 { x: key.1, y: key.0 },
            IVec2 { x, y } if *x == -1 && *y == 0 => IVec2 { x: key.0, y: key.1 },
            IVec2 { x, y } if *x == 0 && *y == -1 => IVec2 { x: key.1, y: key.0 },
            _ => unreachable!(),
        };

        match input.get(&pos) {
            None | Some(false) => continue,
            Some(true) => {}
        }

        // we have a round rock at `key`
        let mut test_pos = pos + *dir;
        while !(test_pos.x < 0 || test_pos.y < 0 || test_pos.x >= dim.x || test_pos.y >= dim.y) {
            match input.get(&test_pos) {
                None => {}
                Some(_) => break,
            }

            test_pos += *dir;
        }

        // correct off by one
        test_pos -= *dir;

        if test_pos == pos {
            continue;
        }
        // remove old entry
        assert_eq!(input.remove(&pos), Some(true));
        assert_eq!(input.insert(test_pos, true), None);
    }
}

#[tracing::instrument(skip(map))]
fn score_map(map: &FxHashMap<IVec2, bool>, hight: i32) -> u32 {
    map.iter()
        .filter_map(|(pos, val)| if !val { None } else { Some(hight - pos.y) })
        .sum::<i32>() as u32
}

#[aoc(day14, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    let mut map = input.0.to_owned();
    tilt(&mut map, &input.1, &IVec2 { x: 0, y: -1 });

    score_map(&map, input.1.y)
}

#[aoc(day14, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    let mut map = input.0.to_owned();

    let map_to_key = |map: &FxHashMap<IVec2, bool>| -> String {
        let mut output = String::new();
        for y in 0..input.1.y {
            for x in 0..input.1.x {
                output.push(match map.get(&IVec2 { x, y }) {
                    None => '.',
                    Some(true) => 'O',
                    Some(false) => '#',
                });
            }
        }
        output
    };

    let max_rounds = 1_000_000_000;
    // let mut history: HashMap<String, (usize, u32)> = HashMap::new();
    let mut history: FxHashMap<String, (usize, u32)> = FxHashMap::default();
    for idx in 0..max_rounds {
        // spin!
        tilt(&mut map, &input.1, &IVec2 { x: 0, y: -1 });
        tilt(&mut map, &input.1, &IVec2 { x: -1, y: 0 });
        tilt(&mut map, &input.1, &IVec2 { x: 0, y: 1 });
        tilt(&mut map, &input.1, &IVec2 { x: 1, y: 0 });

        // insert and check for loop
        if let Some((idx_start, _)) =
            history.insert(map_to_key(&map), (idx, score_map(&map, input.1.y)))
        {
            // found loop
            let len = idx - idx_start;
            let target_idx = idx_start + (max_rounds - idx_start) % len - 1;

            info!("found loop at {idx}, starting at {idx_start}, len {len}");

            return history
                .iter()
                .find(|(_, (idx, _))| target_idx == *idx)
                .unwrap()
                .1
                 .1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use super::{input_generator, part1, part2, tilt};

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 136);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 64);
    }

    #[test_log::test]
    fn test3() {
        let (map1, _) = input_generator(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        );
        let (map2, _) = input_generator(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        );
        let (map3, _) = input_generator(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );

        let input = input_generator(INPUT);
        let mut map = input.0.to_owned();

        tilt(&mut map, &input.1, &IVec2 { x: 0, y: -1 });
        tilt(&mut map, &input.1, &IVec2 { x: -1, y: 0 });
        tilt(&mut map, &input.1, &IVec2 { x: 0, y: 1 });
        tilt(&mut map, &input.1, &IVec2 { x: 1, y: 0 });
        assert_eq!(map, map1);

        tilt(&mut map, &input.1, &IVec2 { x: 0, y: -1 });
        tilt(&mut map, &input.1, &IVec2 { x: -1, y: 0 });
        tilt(&mut map, &input.1, &IVec2 { x: 0, y: 1 });
        tilt(&mut map, &input.1, &IVec2 { x: 1, y: 0 });
        assert_eq!(map, map2);

        tilt(&mut map, &input.1, &IVec2 { x: 0, y: -1 });
        tilt(&mut map, &input.1, &IVec2 { x: -1, y: 0 });
        tilt(&mut map, &input.1, &IVec2 { x: 0, y: 1 });
        tilt(&mut map, &input.1, &IVec2 { x: 1, y: 0 });
        assert_eq!(map, map3);
    }
}
