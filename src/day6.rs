use aoc_runner_derive::{aoc, aoc_generator};

type T = Race;

pub struct Race {
    duration: u32,
    best: u32,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<T> {
    let (time, distance) = input.split_once('\n').unwrap();

    debug_assert!(time.starts_with("Time:"));
    debug_assert!(distance.starts_with("Distance:"));

    let read_line = |input: &str| -> Vec<u32> {
        let (_, times) = input.split_once(':').unwrap();
        times
            .trim()
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect()
    };

    let time = read_line(time);
    let distance = read_line(distance);

    time.into_iter()
        .zip(distance.into_iter())
        .map(|(duration, best)| Race { best, duration })
        .collect()
}

fn calc(duration: i64, best: i64) -> u32 {
    let p = duration as f64 / 2.0;
    let q = best as f64 + 0.1f64.powi(5); // little offset for part 1
    let sqrt = (p.powi(2) - q).sqrt();

    let x1 = (-p + sqrt) * -1.0;
    let x2 = (-p - sqrt) * -1.0;

    let x1 = x1.ceil() as u32;
    let x2 = x2.floor() as u32;

    x2 - x1 + 1
}

#[aoc(day6, part1)]
pub fn part1(input: &[T]) -> u32 {
    input
        .iter()
        .map(|race| calc(race.duration as i64, race.best as i64))
        .reduce(|x, y| x * y)
        .unwrap()
}

#[aoc(day6, part2)]
pub fn part2(input: &[T]) -> u32 {
    let (duration, best): (i64, i64) = input
        .iter()
        .map(|r| (r.duration.to_string(), r.best.to_string()))
        .reduce(|(a1, a2), (b1, b2)| (a1 + &b1, a2 + &b2))
        .map(|(duration, best)| (duration.parse().unwrap(), best.parse().unwrap()))
        .unwrap();

    calc(duration, best)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 288);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 71503);
    }
}
