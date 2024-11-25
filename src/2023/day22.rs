use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

type T = (usize, usize);

#[aoc_generator(day22)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let mut bricks = input
        .lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) = l
                .split(|c: char| !c.is_ascii_digit())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (x1, y1, z1, x2, y2, z2)
        })
        .collect::<Vec<_>>();
    bricks.sort_by_key(|b| b.2);
    let mut space = FxHashMap::default();
    for (i, b) in bricks.iter_mut().enumerate() {
        let (x1, y1, mut z1, x2, y2, mut z2) = *b;
        while z1 > 1
            && (x1..=x2)
                .cartesian_product(y1..=y2)
                .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
        {
            z2 -= 1;
            z1 -= 1;
        }
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    space.insert((x, y, z), i);
                }
            }
        }
        *b = (x1, y1, z1, x2, y2, z2);
    }
    let mut above = FxHashMap::<_, FxHashSet<_>>::default();
    let mut below = FxHashMap::<_, FxHashSet<_>>::default();
    for (i, &(x1, y1, z1, x2, y2, _)) in bricks.iter().enumerate() {
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            if let Some(&j) = space.get(&(x, y, z1 - 1)) {
                above.entry(j).or_default().insert(i);
                below.entry(i).or_default().insert(j);
            }
        }
    }
    let (mut p1, mut p2) = (0, 0);
    for b in 0..bricks.len() {
        let mut falling = FxHashSet::default();
        disintegrate_all(&above, &below, &mut falling, b);
        p1 += (falling.len() == 1) as usize;
        p2 += falling.len() - 1;
    }
    (p1, p2)
}

fn disintegrate_all(
    above: &FxHashMap<usize, FxHashSet<usize>>,
    below: &FxHashMap<usize, FxHashSet<usize>>,
    falling: &mut FxHashSet<usize>,
    brick: usize,
) {
    if !falling.insert(brick) {
        return;
    }
    let Some(parents) = above.get(&brick) else {
        return;
    };
    for &parent in parents {
        if below[&parent].iter().all(|x| falling.contains(x)) {
            disintegrate_all(above, below, falling, parent);
        }
    }
}

#[aoc(day22, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    input.0 as u32
}

#[aoc(day22, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    input.1 as u32
}

// thanks https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/22.rs

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 5);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 7);
    }
}
