use aoc_runner_derive::{aoc, aoc_generator};
use glam::I64Vec2;

#[derive(Debug)]
pub struct Instruction {
    dir: I64Vec2,
    count: i64,

    part2: (I64Vec2, i64),
}

type T = Instruction;

#[aoc_generator(day18)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            let dir = match it.next().unwrap() {
                "U" => I64Vec2::NEG_Y,
                "D" => I64Vec2::Y,
                "L" => I64Vec2::NEG_X,
                "R" => I64Vec2::X,
                _ => unreachable!(),
            };
            let count = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();

            let (dist, c) = color[2..color.len() - 1].split_at(5);
            let dist2 = i64::from_str_radix(&dist, 16).unwrap();
            let dir2 = match c {
                "0" => I64Vec2::X,
                "1" => I64Vec2::Y,
                "2" => I64Vec2::NEG_X,
                "3" => I64Vec2::NEG_Y,
                _ => unreachable!(),
            };

            Instruction {
                dir,
                count,
                part2: (dir2, dist2),
            }
        })
        .collect()
}

fn shoe_lace(lines: &Vec<I64Vec2>) -> i64 {
    let mut ret: i64 = 0;

    for lines in lines.windows(2) {
        let a = &lines[0];
        let b = &lines[1];

        ret += a.y * b.x - a.x * b.y;
    }

    ret.abs() / 2
}

#[aoc(day18, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &[T]) -> i64 {
    let mut border = 0;
    let mut lines = vec![I64Vec2::ZERO];
    for inst in input {
        let dist = inst.count;
        let dir = inst.dir;
        let prev = lines.last().unwrap();

        border += dist;
        lines.push(*prev + dir * dist);
    }

    shoe_lace(&lines) + border / 2 + 1
}

#[aoc(day18, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &[T]) -> i64 {
    let mut border = 0;
    let mut lines = vec![I64Vec2::ZERO];
    for inst in input {
        let dir = inst.part2.0;
        let dist = inst.part2.1;
        let prev = lines.last().unwrap();

        border += dist;
        lines.push(*prev + dir * dist);
    }

    shoe_lace(&lines) + border / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 62);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 952408144115);
    }
}
