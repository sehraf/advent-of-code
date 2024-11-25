use aoc_runner_derive::{aoc, aoc_generator};

type T = Card;

pub struct Card {
    // win: Vec<u32>,
    // you: Vec<u32>,
    matches: u32,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            let (_card, rest) = line.split_once(':').unwrap();

            let (win, you) = rest.split_once('|').unwrap();
            let win: Vec<u32> = win
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let you: Vec<u32> = you
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            let matches = you.iter().filter(|y| win.contains(y)).count() as u32;
            Card { matches }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[T]) -> u32 {
    input
        .iter()
        .map(|card| card.matches)
        .map(|matches| if matches == 0 { 0 } else { 1u32 << matches - 1 })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[T]) -> u32 {
    // double the size to skip the `if`
    let mut cards = vec![1; input.len() * 2];

    input.iter().enumerate().for_each(|(index, card)| {
        for i in index..index + card.matches as usize {
            // not necessary due to large `cards`
            // if i + 1 < input.len() {
            cards[i + 1] += cards[index];
            // }
        }
    });

    cards.iter().take(input.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 30);
    }
}
