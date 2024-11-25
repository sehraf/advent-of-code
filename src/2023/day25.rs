use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};

type T = FxHashMap<u16, FxHashSet<u16>>;

#[aoc_generator(day25)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    let mut node_ids = FxHashMap::default();
    let mut nodes = FxHashMap::default();

    input.lines().for_each(|line| {
        let (start, ends) = line.split_once(": ").unwrap();
        let ends: Vec<_> = ends.split(' ').collect();
        for &end in ends.iter() {
            let id = node_ids.len() as u16;
            node_ids.entry(end).or_insert(id);
        }
        let id = node_ids.len() as u16;
        node_ids.entry(start).or_insert(id);

        let start = node_ids[&start];
        let ends = ends.iter().map(|&e| node_ids[&e]);
        for end in ends.clone() {
            nodes
                .entry(end)
                .or_insert_with(FxHashSet::default)
                .insert(start);
        }
        nodes
            .entry(start)
            .or_insert_with(FxHashSet::default)
            .extend(ends);
    });

    nodes
}

#[tracing::instrument(skip(nodes))]
fn connected_count(nodes: &T) -> usize {
    let mut visited = FxHashSet::default();
    let mut queue = Vec::new();
    queue.push(*nodes.keys().next().unwrap());
    while let Some(node) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }
        for &neighbor in nodes[&node].iter() {
            queue.push(neighbor);
        }
    }
    visited.len()
}

#[tracing::instrument(skip(nodes))]
fn pathfind(nodes: &T, start: u16, end: u16) -> Option<Vec<u16>> {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    let mut parents = FxHashMap::default();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        if node == end {
            break;
        }
        for &neighbor in nodes[&node].iter() {
            if !visited.contains(&neighbor) {
                parents.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }

    let mut path = Vec::new();
    let mut node = end;
    while node != start {
        path.push(node);
        if parents.contains_key(&node) {
            node = parents[&node];
        } else {
            return None;
        }
    }
    path.push(start);
    path.reverse();
    Some(path)
}

#[aoc(day25, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> usize {
    let mut components = input.to_owned();

    for i in 1..components.len() {
        let paths = (0..3)
            .map(|_| {
                let path = pathfind(&components, 0, i as u16).unwrap();
                path.windows(2).for_each(|e| {
                    components.get_mut(&e[0]).unwrap().remove(&e[1]);
                    components.get_mut(&e[1]).unwrap().remove(&e[0]);
                });
                path
            })
            .collect::<Vec<_>>();

        match pathfind(&components, 0, i as u16) {
            // There is still a path, the components are in the same group
            Some(_) => (),
            // All 3 connecting edges have been removed, the components are in different groups
            None => {
                let size1 = connected_count(&components);
                let size2 = components.len() - size1;
                return size1 * size2;
            }
        }

        paths.into_iter().for_each(|path| {
            path.windows(2).for_each(|e| {
                components.get_mut(&e[0]).unwrap().insert(e[1]);
                components.get_mut(&e[1]).unwrap().insert(e[0]);
            });
        })
    }

    0
}

#[aoc(day25, part2)]
#[tracing::instrument(skip(_input))]
pub fn part2(_input: &T) -> u32 {
    0
}

// thanks https://github.com/Zemogus/AOC-2023/blob/main/src/day25.rs

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 54);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }
}
