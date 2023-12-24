use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use z3::ast::{Ast, Int};

struct Hailstone<T> {
    pos: (T, T, T),
    vel: (T, T, T),
}

impl Hailstone<i128> {
    fn to_line(&self) -> (i128, i128, i128) {
        let (x1, y1, _) = self.pos;
        let (vx, vy, _) = self.vel;
        let a = vy;
        let b = -vx;
        let c = vx * y1 - vy * x1;
        (a, b, c)
    }
}

type T = Hailstone<i128>;

#[aoc_generator(day24)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let pos: (i128, i128, i128) = pos
                .split(", ")
                .map(str::trim)
                .map(|c| c.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let vel: (i128, i128, i128) = vel
                .split(", ")
                .map(str::trim)
                .map(|c| c.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Hailstone {
                pos: (pos.0, pos.1, pos.2),
                vel: (vel.0, vel.1, vel.2),
            }
        })
        .collect()
}

#[tracing::instrument]
fn intersection(
    (a1, b1, c1): (i128, i128, i128),
    (a2, b2, c2): (i128, i128, i128),
) -> Option<(i128, i128)> {
    if (a1 * b2 - a2 * b1) == 0 {
        return None;
    }
    let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
    let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

    Some((x, y))
}

#[aoc(day24, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &[T]) -> u32 {
    let hailstones = input;

    #[cfg(not(test))]
    const MIN: i128 = 200_000_000_000_000;
    #[cfg(not(test))]
    const MAX: i128 = 400_000_000_000_000;

    #[cfg(test)]
    const MIN: i128 = 7;
    #[cfg(test)]
    const MAX: i128 = 27;

    let lines = hailstones.iter().map(|h| h.to_line()).collect::<Vec<_>>();
    let mut total = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let abc1 = lines[i];
            let abc2 = lines[j];
            if let Some((x, y)) = intersection(abc1, abc2) {
                if (x - hailstones[i].pos.0).signum() != (hailstones[i].vel.0).signum()
                    || (x - hailstones[j].pos.0).signum() != (hailstones[j].vel.0).signum()
                    || (y - hailstones[i].pos.1).signum() != (hailstones[i].vel.1).signum()
                    || (y - hailstones[j].pos.1).signum() != (hailstones[j].vel.1).signum()
                {
                    continue;
                }

                if x >= MIN && x <= MAX && y >= MIN && y <= MAX {
                    total += 1;
                }
            }
        }
    }

    total
}

#[aoc(day24, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &[T]) -> u32 {
    let hailstones = input;

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let s = z3::Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for (i, hs) in hailstones.iter().take(3).enumerate() {
        let a = Int::from_i64(&ctx, hs.pos.0 as i64);
        let va = Int::from_i64(&ctx, hs.vel.0 as i64);
        let b = Int::from_i64(&ctx, hs.pos.1 as i64);
        let vb = Int::from_i64(&ctx, hs.vel.1 as i64);
        let c = Int::from_i64(&ctx, hs.pos.2 as i64);
        let vc = Int::from_i64(&ctx, hs.vel.2 as i64);

        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.gt(&Int::from_i64(&ctx, 0)));
        s.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        s.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        s.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(x + y + z), true).unwrap();
    res.as_i64().unwrap() as u32

}

// thanks https://gist.github.com/WaterFace/1240609d0d4e15fa4ade3e471e7b501e

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 2);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 47);
    }
}
