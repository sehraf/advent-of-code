use std::{collections::VecDeque, path::PathBuf, vec};

use crate::AdventOfCode;

const DAY: &str = "day21";

struct FairDice {
    last: u64,
}

impl FairDice {
    fn next(&mut self) -> u64 {
        self.last %= 100;
        self.last += 1;
        self.last
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    position: (u8, u8),
    score: (u64, u64),
    player_one: bool,
}

impl Game {
    fn step(&mut self, roll: u64) {
        if self.player_one {
            self.position.0 = ((self.position.0 as u64 + roll - 1) % 10 + 1) as u8;
            self.score.0 += self.position.0 as u64;
        } else {
            self.position.1 = ((self.position.1 as u64 + roll - 1) % 10 + 1) as u8;
            self.score.1 += self.position.1 as u64;
        }
        self.player_one = !self.player_one;
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: (u8, u8),
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1();

        self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle2();

        (a, b)
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String) {
        let input_file = base_dir.join(test_input);
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let mut lines = input.lines();
        let a = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        self.input = (a, b);
    }

    fn puzzle1(&mut self) -> u64 {
        let mut g = Game {
            position: self.input,
            score: (0, 0),
            player_one: true,
        };

        let mut dice = FairDice { last: 0 };
        let mut rolls = 0;
        while g.score.0 < 1000 && g.score.1 < 1000 {
            let steps = (0..3).map(|_| dice.next()).sum();

            g.step(steps);

            rolls += 3;
        }

        rolls
            * if g.score.0 >= 1000 {
                g.score.1
            } else {
                g.score.0
            }
    }

    fn puzzle2(&mut self) -> u64 {
        let mut games = VecDeque::new();
        games.push_back((
            Game {
                position: self.input,
                score: (0, 0),
                player_one: true,
            },
            1,
        ));

        let mut wins = (0, 0);
        let times = vec![1, 3, 6, 7, 6, 3, 1];

        while let Some((g, num)) = games.pop_front() {
            (3..=9).for_each(|roll| {
                let mut g = g.clone();
                g.step(roll);

                let num = num * times[roll as usize - 3];
                if g.score.0 >= 21 {
                    wins.0 += num;
                } else if g.score.1 >= 21 {
                    wins.1 += num;
                } else {
                    games.push_back((g, num));
                }
            });
        }

        if wins.0 > wins.1 {
            wins.0
        } else {
            wins.1
        }
    }
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::{Data, DAY};

    #[test]
    fn puzzle1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle1(), 739785);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 444356092776315);
    }
}
