use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use glam::IVec2;
use itertools::Itertools;

pub enum Tile {
    SplitterVertical,   // |
    SplitterHorizontal, // -
    MirrorLeft,         // \
    MirrorRight,        // /
}

type T = (FxHashMap<IVec2, Tile>, IVec2);

#[aoc_generator(day16)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let y = input.lines().count() as i32;
    let x = input.lines().next().unwrap().len() as i32;

    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices().filter_map(move |(x, c)| {
                    let pos = IVec2 {
                        x: x as i32,
                        y: y as i32,
                    };
                    match c {
                        '|' => Some((pos, Tile::SplitterVertical)),
                        '-' => Some((pos, Tile::SplitterHorizontal)),
                        '\\' => Some((pos, Tile::MirrorLeft)),
                        '/' => Some((pos, Tile::MirrorRight)),
                        '.' => None,
                        _ => unreachable!(),
                    }
                })
            })
            .collect(),
        IVec2 { x, y },
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Beam {
    pos: IVec2,
    dir: IVec2,
}

fn process_beam(map: &FxHashMap<IVec2, Tile>, dim: IVec2, start_beam: Beam) -> u32 {
    let mut energized = FxHashSet::default();
    let mut beams = vec![start_beam];

    while let Some(mut beam) = beams.pop() {
        loop {
            // known position?
            if energized.contains(&beam) {
                break;
            }
            // left map?
            if beam.pos.x < 0 || beam.pos.y < 0 || beam.pos.x >= dim.x || beam.pos.y >= dim.y {
                break;
            }

            energized.insert(beam);

            match map.get(&beam.pos) {
                None => beam.pos += beam.dir,
                Some(tile) => match (tile, beam.dir) {
                    (Tile::MirrorLeft, IVec2::X) => {
                        beam.dir = IVec2::Y;
                        beam.pos += IVec2::Y
                    }
                    (Tile::MirrorLeft, IVec2::Y) => {
                        beam.dir = IVec2::X;
                        beam.pos += IVec2::X
                    }
                    (Tile::MirrorLeft, IVec2::NEG_X) => {
                        beam.dir = IVec2::NEG_Y;
                        beam.pos += IVec2::NEG_Y
                    }
                    (Tile::MirrorLeft, IVec2::NEG_Y) => {
                        beam.dir = IVec2::NEG_X;
                        beam.pos += IVec2::NEG_X
                    }

                    (Tile::MirrorRight, IVec2::X) => {
                        beam.dir = IVec2::NEG_Y;
                        beam.pos += IVec2::NEG_Y
                    }
                    (Tile::MirrorRight, IVec2::Y) => {
                        beam.dir = IVec2::NEG_X;
                        beam.pos += IVec2::NEG_X
                    }
                    (Tile::MirrorRight, IVec2::NEG_X) => {
                        beam.dir = IVec2::Y;
                        beam.pos += IVec2::Y
                    }
                    (Tile::MirrorRight, IVec2::NEG_Y) => {
                        beam.dir = IVec2::X;
                        beam.pos += IVec2::X
                    }

                    (Tile::SplitterHorizontal, IVec2::X)
                    | (Tile::SplitterHorizontal, IVec2::NEG_X) => beam.pos += beam.dir,
                    (Tile::SplitterHorizontal, IVec2::Y)
                    | (Tile::SplitterHorizontal, IVec2::NEG_Y) => {
                        beams.push(Beam {
                            pos: beam.pos + IVec2::NEG_X,
                            dir: IVec2::NEG_X,
                        });
                        beam.dir = IVec2::X;
                        beam.pos += IVec2::X
                    }

                    (Tile::SplitterVertical, IVec2::Y) | (Tile::SplitterVertical, IVec2::NEG_Y) => {
                        beam.pos += beam.dir
                    }
                    (Tile::SplitterVertical, IVec2::X) | (Tile::SplitterVertical, IVec2::NEG_X) => {
                        beams.push(Beam {
                            pos: beam.pos + IVec2::NEG_Y,
                            dir: IVec2::NEG_Y,
                        });
                        beam.dir = IVec2::Y;
                        beam.pos += IVec2::Y
                    }

                    _ => unreachable!(),
                },
            }
        }
    }

    energized.iter().unique_by(|b| b.pos).count() as u32
}

#[aoc(day16, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    let start_beam = Beam {
        pos: IVec2 { x: 0, y: 0 },
        dir: IVec2::X,
    };
    let map = &input.0;
    let dim = input.1;

    process_beam(map, dim, start_beam)
}

#[aoc(day16, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    let map = &input.0;
    let dim = input.1;

    let start_positions = [
        Beam {
            pos: IVec2 { x: 0, y: 0 },
            dir: IVec2::X,
        },
        Beam {
            pos: IVec2 { x: 0, y: 0 },
            dir: IVec2::Y,
        },
        Beam {
            pos: IVec2 { x: dim.x - 1, y: 0 },
            dir: IVec2::NEG_X,
        },
        Beam {
            pos: IVec2 { x: dim.x - 1, y: 0 },
            dir: IVec2::Y,
        },
        Beam {
            pos: IVec2 { x: 0, y: dim.y - 1 },
            dir: IVec2::X,
        },
        Beam {
            pos: IVec2 { x: 0, y: dim.y - 1 },
            dir: IVec2::NEG_Y,
        },
        Beam {
            pos: IVec2 {
                x: dim.x - 1,
                y: dim.y - 1,
            },
            dir: IVec2::NEG_X,
        },
        Beam {
            pos: IVec2 {
                x: dim.x - 1,
                y: dim.y - 1,
            },
            dir: IVec2::NEG_Y,
        },
    ]
    .into_iter()
    .chain((1..dim.x - 1).map(|x| Beam {
        pos: IVec2 { x: x, y: 0 },
        dir: IVec2::Y,
    }))
    .chain((1..dim.x - 1).map(|x| Beam {
        pos: IVec2 { x: x, y: dim.y - 1 },
        dir: IVec2::NEG_Y,
    }))
    .chain((1..dim.y - 1).map(|y| Beam {
        pos: IVec2 { x: 0, y: y },
        dir: IVec2::X,
    }))
    .chain((1..dim.y - 1).map(|y| Beam {
        pos: IVec2 { x: dim.x - 1, y: y },
        dir: IVec2::NEG_X,
    }));

    start_positions
        .map(|start_beam| process_beam(map, dim, start_beam))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 46);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 51);
    }
}
