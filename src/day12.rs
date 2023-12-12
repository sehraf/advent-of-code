use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Operational, // .
    Broken,      // #
    Unknown,     // ?
}

type T = (Vec<State>, Vec<usize>);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();

            (
                springs
                    .chars()
                    .map(|c| match c {
                        '.' => State::Operational,
                        '#' => State::Broken,
                        '?' => State::Unknown,
                        _ => unreachable!(),
                    })
                    .collect(),
                groups.split(',').filter_map(|g| g.parse().ok()).collect(),
            )
        })
        .collect()
}

fn possible_ways(
    history: &mut HashMap<(usize, usize, usize), u64>,
    springs: &[State],
    span: Option<usize>,
    groups: &[usize],
) -> u64 {
    // end of springs == end of group?
    if springs.is_empty() {
        return match (span, groups.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == groups[0] => 1,
            _ => 0,
        };
    }
    // withing a span but no groups left?
    if span.is_some() && groups.is_empty() {
        return 0;
    }

    // have we been here before?
    let key = (springs.len(), span.unwrap_or(0), groups.len());
    if let Some(&x) = history.get(&key) {
        return x;
    }

    // we need to go deeper!
    let ways = match (springs[0], span) {
        (State::Operational, Some(x)) if x != groups[0] => 0,
        (State::Operational, Some(_)) => possible_ways(history, &springs[1..], None, &groups[1..]),
        (State::Operational, None) => possible_ways(history, &springs[1..], None, groups),
        (State::Broken, Some(x)) => possible_ways(history, &springs[1..], Some(x + 1), groups),
        (State::Broken, None) => possible_ways(history, &springs[1..], Some(1), groups),
        (State::Unknown, Some(x)) => {
            let mut ans = possible_ways(history, &springs[1..], Some(x + 1), groups);
            if x == groups[0] {
                ans += possible_ways(history, &springs[1..], None, &groups[1..])
            }
            ans
        }
        (State::Unknown, None) => {
            possible_ways(history, &springs[1..], Some(1), groups)
                + possible_ways(history, &springs[1..], None, groups)
        }
    };
    history.insert(key, ways);
    ways
}

#[aoc(day12, part1)]
pub fn part1(input: &[T]) -> u64 {
    let mut history = HashMap::new();

    input
        .iter()
        .map(|(springs, groups)| {
            history.clear();
            possible_ways(&mut history, springs, None, groups)
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &[T]) -> u64 {
    let mut history = HashMap::new();

    input
        .iter()
        .map(|(springs, groups)| {
            // can we do better/nicer?!
            let new_springs = (0..5).map(|_| springs).fold(vec![], |acc, cur| {
                let mut acc = acc.to_owned();
                if !acc.is_empty() {
                    acc.push(State::Unknown);
                }
                acc.append(&mut cur.to_owned());
                acc
            });
            let new_groups = (0..5).flat_map(|_| groups).copied().collect::<Vec<_>>();

            history.clear();
            possible_ways(&mut history, &new_springs, None, &new_groups)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn test2() {
        // 3350142103 too low
        assert_eq!(part2(&input_generator(INPUT)), 525152);
    }
}
