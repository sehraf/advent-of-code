use aoc_runner_derive::aoc;

fn do_it(input: &str, len: usize) -> u32 {
    for (i, section) in input.chars().collect::<Vec<_>>().windows(len).enumerate() {
        let mut passed = 0;
        for c in section {
            if section.iter().filter(|&c2| c != c2).count() == len - 1 {
                passed += 1;
            }
        }
        if passed == len {
            return (i + len) as u32;
        }
    }

    unreachable!()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    do_it(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    do_it(input, 14)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 19);
    }
}
