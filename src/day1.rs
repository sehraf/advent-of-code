use aoc_runner_derive::{aoc, aoc_generator};

type T = u32;

fn read_input(input: &str, include_words: bool) -> Vec<T> {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .lines()
        .map(|line| {
            let mut found = vec![];

            for index in 0..line.len() {
                let c = line.chars().nth(index).unwrap();
                let subs = &line[index..];

                // number
                if let Some(n) = c.to_digit(10) {
                    found.push(n);
                    continue;
                }

                // written out numbers
                if include_words {
                    for (s, v) in numbers {
                        if subs.starts_with(s) {
                            found.push(v);
                            continue;
                        }
                    }
                }
            }

            let x = found.first().unwrap();
            let y = found.last().unwrap();
            x * 10 + y
        })
        .collect()
}

#[aoc_generator(day1, part1)]
pub fn input_generator1(input: &str) -> Vec<T> {
    read_input(input, false)
}

#[aoc_generator(day1, part2)]
pub fn input_generator2(input: &str) -> Vec<T> {
    read_input(input, true)
}

#[aoc(day1, part1)]
pub fn part1(input: &[T]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[T]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator1, input_generator2, part1, part2};

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator1(INPUT)), 142);
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(&input_generator2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )),
            281
        );
    }
}
