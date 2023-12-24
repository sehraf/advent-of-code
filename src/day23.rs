use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use itertools::Itertools;

type T = Vec<u8>;

#[aoc_generator(day23)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> Vec<T> {
    let x = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    x
}

const NEIGHBORS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[allow(clippy::type_complexity)]
fn dfs(
    graph: &FxHashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
) -> Option<usize> {
    if r == seen.len() - 1 {
        return Some(0);
    }
    let mut max_dist = None;
    for &(rr, cc, d) in &graph[&(r, c)] {
        if !seen[rr][cc] {
            seen[rr][cc] = true;
            if let Some(dist) = dfs(graph, seen, (rr, cc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[rr][cc] = false;
        }
    }
    max_dist
}

fn solve(grid: &[Vec<u8>], part2: bool) -> usize {
    let mut graph = FxHashMap::<_, Vec<_>>::default();
    for (r, c) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let neighbors = match grid[r][c] {
            b'#' => continue,
            _ if part2 => NEIGHBORS,
            b'.' => NEIGHBORS,
            b'^' => &NEIGHBORS[0..][..1],
            b'>' => &NEIGHBORS[1..][..1],
            b'v' => &NEIGHBORS[2..][..1],
            b'<' => &NEIGHBORS[3..][..1],
            _ => unreachable!(),
        };
        let e = graph.entry((r, c)).or_default();
        for (dr, dc) in neighbors {
            let rr = (r as isize + dr) as usize;
            let cc = (c as isize + dc) as usize;
            let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                continue;
            };
            if tile != b'#' {
                e.push((rr, cc, 1));
            }
        }
    }
    let corridors = graph
        .iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();
    for (r, c) in corridors {
        let neighbors = graph.remove(&(r, c)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let n1 = graph.get_mut(&(r1, c1)).unwrap();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (r2, c2, d1 + d2);
        }
        let n2 = graph.get_mut(&(r2, c2)).unwrap();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (r1, c1, d1 + d2);
        }
    }
    dfs(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap()
}

#[aoc(day23, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &[T]) -> u32 {
    solve(input, false) as u32
}

#[aoc(day23, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &[T]) -> u32 {
    solve(input, true) as u32
}

// thanks https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 94);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 154);
    }
}
