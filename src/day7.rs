use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::{Ordering, PartialOrd};

type T = (Vec<char>, u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    // reverse order so we can use `derive` macro
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Card<const PART2: bool>(char);

impl<const PART2: bool> Ord for Card<PART2> {
    fn cmp(&self, other: &Self) -> Ordering {
        let chars = if !PART2 {
            vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ]
        } else {
            vec![
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ]
        };

        let p1 = chars.iter().position(|x| x == &self.0).unwrap();
        let p2 = chars.iter().position(|x| x == &other.0).unwrap();

        // less is better
        p2.cmp(&p1)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Hand<const PART2: bool> {
    cards: Vec<Card<PART2>>,
    bid: u32,

    ty: Classification,
}

impl<const PART2: bool> Ord for Hand<PART2> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Equal => {
                // compare cards
                match self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|(a, b)| a != b)
                {
                    None => Ordering::Equal,
                    Some((a, b)) => a.cmp(b),
                }
            }
            o => o,
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_at(5);

            let cards: Vec<char> = cards.chars().collect();
            debug_assert_eq!(cards.len(), 5);

            (cards, bid.trim().parse().unwrap())
        })
        .collect()
}

fn to_classification(same: &Vec<usize>) -> Classification {
    if same.contains(&5) {
        Classification::FiveOfAKind
    } else if same.contains(&4) {
        Classification::FourOfAKind
    } else if same.contains(&3) && same.contains(&2) {
        Classification::FullHouse
    } else if same.contains(&3) {
        Classification::ThreeOfAKind
    } else if same.iter().filter(|&e| e == &2).count() >= 2 {
        Classification::TwoPair
    } else if same.contains(&2) {
        Classification::OnePair
    } else if same.contains(&1) {
        Classification::HighCard
    } else {
        // this is an edge case when all cards are Js, only relevant for part 2
        // Changing the counting logic turned out harder than this hack
        assert_eq!(same.iter().sum::<usize>(), 0);
        Classification::FiveOfAKind
    }
}

fn input_hands<const PART2: bool>(
    input: &[T],
    classify: fn(&Vec<Card<PART2>>) -> Classification,
) -> Vec<Hand<PART2>> {
    input
        .iter()
        .map(|(cards, bid)| {
            let cards = cards.iter().map(|&c| Card(c)).collect();
            let class = classify(&cards);

            Hand {
                cards,
                bid: *bid,

                ty: class,
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[T]) -> u32 {
    let classify = |cards: &Vec<Card<false>>| -> Classification {
        let same = cards
            .iter()
            .map(|c| c.0)
            // count the occurrence of a card
            .counts()
            .values()
            .cloned()
            .collect();
        to_classification(&same)
    };

    let mut hands = input_hands(input, classify);

    hands.sort_by(|a, b| a.cmp(b));

    hands
        .iter()
        .enumerate()
        .map(|(idx, card)| card.bid * (idx + 1) as u32)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[T]) -> u32 {
    let classify = |cards: &Vec<Card<true>>| -> Classification {
        // it only makes sense to replace J with other existing cards
        let j_candidates: Vec<char> = cards.iter().map(|c| c.0).unique().collect();

        // now check what happens when J is replaced by any of the other cards
        j_candidates
            .iter()
            .map(|j| {
                let a = {
                    let same = cards
                        .iter()
                        .map(|c| c.0)
                        .counts()
                        .values()
                        .cloned()
                        .collect();
                    to_classification(&same)
                };

                // count the occurrence of a card but replace J
                let b = {
                    let same = cards
                        .iter()
                        .map(|c| c.0)
                        .map(|card| if card == 'J' { *j } else { card })
                        .counts()
                        .values()
                        .cloned()
                        .collect();
                    to_classification(&same)
                };
                a.max(b)
            })
            .max()
            .unwrap()
    };

    let mut hands = input_hands(input, classify);

    hands.sort_by(|a, b| a.cmp(b));

    hands
        .iter()
        .enumerate()
        .map(|(idx, card)| card.bid * (idx + 1) as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {

    use std::cmp::Ordering;

    use super::{input_generator, part1, part2, Card, Classification, Hand};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const INPUT2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 6440);
        assert_eq!(part1(&input_generator(INPUT2)), 6592);
    }

    #[test]
    fn test2() {
        // 250073180    too low
        assert_eq!(part2(&input_generator(INPUT)), 5905);
        assert_eq!(part2(&input_generator(INPUT2)), 6839);
    }

    #[test]
    fn cards() {
        let a: Card<false> = Card('2');
        let b: Card<false> = Card('A');

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        let a: Card<false> = Card('J');
        let b: Card<false> = Card('K');

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        let a: Card<true> = Card('J');
        let b: Card<true> = Card('2');

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn hand_type() {
        let a = Classification::HighCard;
        let b = Classification::FiveOfAKind;

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn hand() {
        // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger.
        let a: Hand<false> = Hand {
            bid: 0,
            cards: { "33332".chars().map(|c| Card(c)).collect() },
            ty: Classification::FourOfAKind,
        };
        let b: Hand<false> = Hand {
            bid: 0,
            cards: { "2AAAA".chars().map(|c| Card(c)).collect() },
            ty: Classification::FourOfAKind,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        // Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger
        let a: Hand<false> = Hand {
            bid: 0,
            cards: { "77888".chars().map(|c| Card(c)).collect() },
            ty: Classification::FullHouse,
        };
        let b: Hand<false> = Hand {
            bid: 0,
            cards: { "77788".chars().map(|c| Card(c)).collect() },
            ty: Classification::FullHouse,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        // JKKK2 is weaker than QQQQ2 because J is weaker than Q.
        let a: Hand<true> = Hand {
            bid: 0,
            cards: { "JKKK2".chars().map(|c| Card(c)).collect() },
            ty: Classification::FullHouse,
        };
        let b: Hand<true> = Hand {
            bid: 0,
            cards: { "QQQQ2".chars().map(|c| Card(c)).collect() },
            ty: Classification::FullHouse,
        };
        assert_eq!(a.cmp(&b), Ordering::Less);
    }
}
