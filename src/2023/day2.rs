use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Round {
    red: u32,
    gre: u32,
    blu: u32,
}
#[derive(Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

type T = Game;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<T> {
    parse(input).expect("failed to parse input").1
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Game>> {
    let games = input
        .lines()
        .map(|line| {
            let mut s = line.split(':');
            let game = s.next().unwrap();
            let rest = s.next().unwrap();
            assert!(s.next().is_none());

            let rounds = rest
                .split(';')
                .map(|game| {
                    let mut red = 0;
                    let mut gre = 0;
                    let mut blu = 0;

                    let get = |input: &str, l: usize| -> u32 {
                        input[..input.len() - l].trim().parse().unwrap()
                    };

                    game.split(',').for_each(|c| {
                        if c.ends_with("red") {
                            red = get(c, 4);
                        } else if c.ends_with("green") {
                            gre = get(c, 6);
                        } else if c.ends_with("blue") {
                            blu = get(c, 5);
                        }
                    });

                    Round { red, gre, blu }
                })
                .collect();

            Game {
                id: game[5..].parse().unwrap(),
                rounds,
            }
        })
        .collect();

    Ok((input, games))
}

#[aoc(day2, part1)]
pub fn part1(input: &[T]) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GRE: u32 = 13;
    const MAX_BLU: u32 = 14;

    input
        .into_iter()
        .filter(|&game| {
            game.rounds
                .iter()
                .find(|r| r.red > MAX_RED || r.gre > MAX_GRE || r.blu > MAX_BLU)
                .is_none()
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[T]) -> u32 {
    input
        .into_iter()
        .map(|game| {
            let (red, gre, blu) = game.rounds.iter().fold((0, 0, 0), |(red, gre, blu), r| {
                (
                    cmp::max(red, r.red),
                    cmp::max(gre, r.gre),
                    cmp::max(blu, r.blu),
                )
            });

            red * gre * blu
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 2286);
    }
}
