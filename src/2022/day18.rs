use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec3;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
};

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<IVec3> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

#[aoc(day18, part1)]
pub fn part1(input: &[IVec3]) -> usize {
    let points: HashSet<IVec3> = HashSet::from_iter(input.iter().cloned());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .count()
        })
        .sum::<usize>();

    surface_area
}

fn process_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> usize {
    // number of free sides
    let x_low = IVec3::new(x - 1, y, z);
    let x_high = IVec3::new(x + 1, y, z);
    let y_low = IVec3::new(x, y - 1, z);
    let y_high = IVec3::new(x, y + 1, z);
    let z_low = IVec3::new(x, y, z - 1);
    let z_high = IVec3::new(x, y, z + 1);
    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .map(|ivec| {
            if is_interior_block(&ivec, &points) {
                // (interior wall, exterior wall)
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .map(|(_interior, exterior)| exterior)
        .sum::<usize>()
}

fn is_interior_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> bool {
    let bounded_x_pos = points
        .iter()
        .find(|point| point.x > x && point.y == y && point.z == z)
        .is_some();
    let bounded_x_neg = points
        .iter()
        .find(|point| point.x < x && point.y == y && point.z == z)
        .is_some();
    let bounded_y_pos = points
        .iter()
        .find(|point| point.x == x && point.y > y && point.z == z)
        .is_some();
    let bounded_y_neg = points
        .iter()
        .find(|point| point.x == x && point.y < y && point.z == z)
        .is_some();
    let bounded_z_pos = points
        .iter()
        .find(|point| point.x == x && point.y == y && point.z > z)
        .is_some();
    let bounded_z_neg = points
        .iter()
        .find(|point| point.x == x && point.y == y && point.z < z)
        .is_some();
    [
        bounded_x_pos,
        bounded_x_neg,
        bounded_y_pos,
        bounded_y_neg,
        bounded_z_pos,
        bounded_z_neg,
    ]
    .iter()
    .all(|v| *v)
}

#[aoc(day18, part2)]
pub fn part2(input: &[IVec3]) -> usize {
    let points: HashSet<IVec3> = HashSet::from_iter(input.iter().cloned());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .map(|ivec| {
                    if is_interior_block(&ivec, &points) {
                        let IVec3 { x, y, z } = *ivec;
                        let x_low = IVec3::new(x - 1, y, z);
                        let x_high = IVec3::new(x + 1, y, z);
                        let y_low = IVec3::new(x, y - 1, z);
                        let y_high = IVec3::new(x, y + 1, z);
                        let z_low = IVec3::new(x, y, z - 1);
                        let z_high = IVec3::new(x, y, z + 1);
                        // (interior wall, exterior wall)
                        let is_really_exterior_block =
                            [x_low, x_high, y_low, y_high, z_low, z_high]
                                .iter()
                                .filter(|ivec| points.get(ivec).is_none())
                                .any(|block| process_block(block, &points) >= 1);
                        if is_really_exterior_block {
                            (0, 1)
                        } else {
                            (1, 0)
                        }
                    } else {
                        (0, 1)
                    }
                })
                .map(|(_interior, exterior)| exterior)
                .sum::<usize>()
        })
        .sum::<usize>();

    surface_area
}

fn parse(input: &str) -> nom::IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        map(
            separated_list1(tag(","), map_res(digit1, str::parse)),
            |p| {
                let mut it = p.into_iter();
                IVec3 {
                    x: it.next().unwrap(),
                    y: it.next().unwrap(),
                    z: it.next().unwrap(),
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 64);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 58);
    }
}
